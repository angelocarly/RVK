use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

fn main() {

    let path = Path::new(r"Output/out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
    encoder.set_color( png::ColorType::Rgba );
    encoder.set_depth( png::BitDepth::Eight );
    let data = [ 0, 0, 255, 255, 0, 0, 0, 255 ];

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data( &data ).unwrap(); // Save

    println!("Written image");
}
