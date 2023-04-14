use std::thread;
use glam::Vec3;
use crate::camera::Camera;
use crate::image::ColorSink;

mod image;
mod rays;
mod camera;

fn color_palette( t: f32, a: Vec3, b: Vec3, c: Vec3, d: Vec3 ) -> Vec3 {
    a + b * Vec3::new( f32::cos( 6.28318 * ( c.x * t + d.x ) ), f32::cos( 6.28318 * ( c.y * t + d.y ) ), f32::cos( 6.28318 * ( c.z * t + d.z ) ) )
}

// in: x, y in range [0, 1]
fn calc_pixel( x: f32, y: f32, camera: & Camera, world: & rays::World ) -> image::Color {

    let ray = camera.get_ray( x, y );
    let castresult = world.cast( ray, 500. );

    let mut col = Vec3::new( 0.2, 0.2, 0.2 );
    if let Some( castresult ) = castresult {
        match castresult {
            rays::CastResult::Hit( hit ) => {
                // col = hit.shape.material().color * ( 1. - hit.bounces as f32 / 25. );
                // col = color_palette( hit.bounces as f32 / 1.1 + 1.2 + hit.distance / 2.5, Vec3::new( 0.5, 0.5, 0.5 ), Vec3::new( 0.5, 0.5, 0.5 ), Vec3::new( 1.0, 0.6, 0.3 ), Vec3::new( 0.2, 0.8, 0.3 ) );
                // col = color_palette(hit.distance / 2.5, Vec3::new( 0.5, 0.5, 0.5 ), Vec3::new( 0.5, 0.5, 0.5 ), Vec3::new( 1.0, 0.6, 0.3 ), Vec3::new( 0.2, 0.8, 0.3 ) );
                // col = color_palette(hit.bounces as f32 * 2. + 2., Vec3::new( 0.5, 0.5, 0.5 ), Vec3::new( 0.6, 0.2, 0.5 ), Vec3::new( 0.7, 0.6, 1.0 ), Vec3::new( 0.6, 0.9, 0.3 ) );
                col = color_palette(
                    hit.weight / 10. + 2.,
                    Vec3::new( 0.5, 0.5, 0.5 ),
                    Vec3::new( 0.6, 0.6, 0.3 ),
                    Vec3::new( 0.7, 0.6, 1.0 ),
                    Vec3::new( 0.6, 0.9, 0.3 )
                );
                // col = Vec3::new( 1., 0., 0. ) * ( hit.bounces as f32 / 40. );
                // col = hit.position;
            },
            rays::CastResult::Miss( miss ) => {
                // col = miss.position * ( 1. - miss.bounces as f32 / 25. );
                // col = Vec3::new( 1., 0., 0. ) * ( hit.bounces as f32 / 40. );
                // col = hit.position;
            }
        }
    }

    // Map the color to [0, 255]
    let mut color = col;
    color.x = f32::min(1., f32::max(0., color.x ) );
    color.y = f32::min(1., f32::max(0., color.y ) );
    color.z = f32::min(1., f32::max(0., color.z ) );
    image::Color((color.x * 255.) as u32, (color.y * 255.) as u32, (color.z * 255.) as u32 )
}

fn generation() {

    let width = 512;
    let height = 512;
    let slices = 16;
    let child_block_size = width * height / slices;

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    let time = std::time::Instant::now();
    for i in 0..slices {
        // Spin up another thread
        children.push(thread::spawn(move || -> ColorSink {

            let camera = camera::Camera::new(
                Vec3::new( 0., 0., -2.0 ),
                Vec3::new( 0., 0., 1. ).normalize(),
                Vec3::new( 0., 1., 0. ).normalize(),
                90.,
                width as f32 / height as f32,
                1.
            );

            let world = rays::World::new();

            let mut color_sink = image::ColorSink::new(width, height / slices);

            let total = width * height / slices;
            for x in 0..(width) {
                for ry in 0..(height/slices) {
                    let y = ry + i * height / slices;

                    let mut col = calc_pixel(( x as f32 - 0.25 ) / width as f32, ( y as f32 - 0.25 ) / height as f32, & camera, & world );
                    col += calc_pixel(( x as f32 + 0.25 ) / width as f32, ( y as f32 - 0.25 ) / height as f32, & camera, & world );
                    col += calc_pixel(( x as f32 + 0.25 ) / width as f32, ( y as f32 + 0.25 ) / height as f32, & camera, & world );
                    col += calc_pixel(( x as f32 - 0.25 ) / width as f32, ( y as f32 + 0.25 ) / height as f32, & camera, & world );
                    col /= image::Color(4, 4, 4);

                    color_sink.set_pixel(x, ry, col );
                }
                if x % 3 == 0 {
                    println!("Thread {}: {:.1}%", i, (x * height / slices) as f32 / total as f32 * 100. );
                }
            }
            return color_sink;
        }));
    }

    let mut color_sink = image::ColorSink::new(width, height);
    let mut i = 0;
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let r = child.join();

        // copy the sink into the main sink
        let child_sink = r.unwrap();
        color_sink.set_block( child_block_size * i, child_sink.get_data() );

        i += 1;
    }

    println!("Done in {:.1} seconds", time.elapsed().as_secs_f32() );

    image::write_png_image( color_sink, "Output/out.png" );
}

fn process( cs: &mut ColorSink ) {
    for x in 0..cs.get_width() {
        for y in 0..cs.get_height() {
            let mut col = cs.get_pixel(x, y);
            col.0 = (col.0 as f32 * 1.0) as u32;
            col.1 = (col.1 as f32 * 1.0) as u32;
            col.2 = (col.2 as f32 * 1.0) as u32;
            cs.set_pixel(x, y, col);
        }
    }
}

fn main() {
    // let mut cs = image::read_png_image( "Output/out.png" );
    // process( &mut cs );
    // image::write_png_image( cs, "Output/out2.png" );
    generation();
}
