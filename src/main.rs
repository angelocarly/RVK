use glam::Vec3;
use std::cmp::{max, min};

mod image;

struct Ray {
    origin: Vec3,
    direction: Vec3
}

struct Hit {
    position: Vec3,
    normal: Vec3,
    distance: f32
}

fn sdf_sphere( pos: Vec3, radius: f32 ) -> (f32, Vec3 ) {
    ( pos.length() - radius, pos.normalize() )
}

// in: x, y in range [0, 1]
fn calc_pixel( x: f32, y: f32 ) -> image::Color {

    let pos = Vec3::new( x * 2. - 1., -y * 2. + 1., -1. );
    let ray = Ray { origin: pos, direction: Vec3::new( 0., 0., 1. ) };

    // Raymarching
    let mut t = 0.;
    let mut hit = None;
    for i in 0..100 {
        let delta_t = sdf_sphere( ray.origin + ray.direction * t, 0.5 );
        t += delta_t.0;
        if delta_t.0 < 0.0001 {
            hit = Some( Hit {
                position: ray.origin + ray.direction * t,
                normal: delta_t.1,
                distance: t
            } );
            break;
        }
    }

    let mut col = Vec3::new( 0., 0., 0. );
    if let Some( hit ) = hit {
        // col = Vec3::new( hit.distance, 0., 0. );
        col = hit.normal;
    }

    let mut color = col;
    color.x = f32::min(1., f32::max(0., color.x ) );
    color.y = f32::min(1., f32::max(0., color.y ) );
    color.z = f32::min(1., f32::max(0., color.z ) );
    image::Color((color.x * 255.) as u32, (color.y * 255.) as u32, (color.z * 255.) as u32 )
}

fn main() {

    let width = 500;
    let height = 500;
    let mut color_sink = image::ColorSink::new(width, height);

    for x in 0..(width) {
        for y in 0..(height) {
            color_sink.set_pixel(x, y, calc_pixel(x as f32 / width as f32, y as f32 / width as f32));
        }
    }

    image::write_png_image( color_sink );
}
