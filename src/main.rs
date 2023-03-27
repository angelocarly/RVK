use glam::Vec3;

mod image;
mod rays;

// in: x, y in range [0, 1]
fn calc_pixel( x: f32, y: f32, world: & rays::World ) -> image::Color {

    let mut cam_pos = Vec3::new( 0., 1.5, -2. );
    let mut pix_pos = Vec3::new( x * 2. - 1., -y * 2., 0. );
    let mut direction = (pix_pos - Vec3::new( 0., 0., -8.0 )).normalize();

    let ray = rays::Ray { origin: cam_pos + pix_pos, direction };

    let hit = world.cast( ray );

    let mut col = Vec3::new( 0., 0., 0. );
    if let Some( hit ) = hit {
        col = hit.shape.material().color
    }

    // Map the color to [0, 255]
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

    let world = rays::World::new();

    for x in 0..(width) {
        for y in 0..(height) {
            color_sink.set_pixel(x, y, calc_pixel(x as f32 / width as f32, y as f32 / width as f32, & world ));
        }
    }

    image::write_png_image( color_sink );
}
