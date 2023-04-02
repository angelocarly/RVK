use glam::Vec3;
use crate::camera::Camera;

mod image;
mod rays;
mod camera;

// in: x, y in range [0, 1]
fn calc_pixel( x: f32, y: f32, camera: & Camera, world: & rays::World ) -> image::Color {

    let ray = camera.get_ray( x, y );
    let hit = world.cast( ray );

    let mut col = Vec3::new( 0.2, 0.2, 0.2 );
    if let Some( hit ) = hit {
        col = hit.shape.material().color;
        // col = hit.normal;
    }

    // Map the color to [0, 255]
    let mut color = col;
    color.x = f32::min(1., f32::max(0., color.x ) );
    color.y = f32::min(1., f32::max(0., color.y ) );
    color.z = f32::min(1., f32::max(0., color.z ) );
    image::Color((color.x * 255.) as u32, (color.y * 255.) as u32, (color.z * 255.) as u32 )
}

fn main() {

    let width = 512;
    let height = 512;
    let mut color_sink = image::ColorSink::new(width, height);
    let camera = camera::Camera::new(
        Vec3::new( 0., 0., -4.5 ),
        Vec3::new( 0., 0., 1. ).normalize(),
        Vec3::new( 0., 1., 0. ).normalize(),
        90.,
        width as f32 / height as f32,
        1.
    );

    let world = rays::World::new();

    let total_pixels = width * height;
    let time = std::time::Instant::now();
    println!("Rendering {} pixels", total_pixels);
    for x in 0..(width) {
        for y in 0..(height) {
            color_sink.set_pixel(x, y, calc_pixel(x as f32 / width as f32, y as f32 / width as f32, & camera, & world ));
        }
        if x % 10 == 0
        {
            println!("{:.0}%", (x * height) as f32 / total_pixels as f32 * 100. );
        }
    }
    println!("Done in {:.2} seconds", time.elapsed().as_secs_f32() );

    image::write_png_image( color_sink );
}
