mod image;

fn main() {

    let width = 100;
    let height = 100;
    let mut color_sink = image::ColorSink::new(width, height);

    for x in 0..(width) {
        for y in 0..(height) {
            color_sink.set_pixel(x, y, image::Color(x, y, 0));
        }
    }

    image::write_png_image( color_sink );
}
