use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

#[derive( Clone )]
pub struct Color( pub u32, pub u32, pub u32 );

pub struct ColorSink {
    width: u32,
    height: u32,
    data: Box<[Color]>
}

impl ColorSink {
    pub fn new(width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            panic!("Width and height must be greater than 0.");
        }

        let data = vec![Color(0, 0, 0); (width * height) as usize].into_boxed_slice();
        Self { width, height, data }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of bounds.");
        }

        self.data[(y * self.width + x) as usize] = color;
    }
}

pub fn write_png_image( in_data: ColorSink ) {
    let path = Path::new(r"Output/out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, in_data.width, in_data.height ); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut im_data = vec![ 0; (in_data.width * in_data.height * 3) as usize ].into_boxed_slice();
    for i in 0..(in_data.width * in_data.height) as usize {
        im_data[i * 3 + 0] = in_data.data[i].0 as u8;
        im_data[i * 3 + 1] = in_data.data[i].1 as u8;
        im_data[i * 3 + 2] = in_data.data[i].2 as u8;
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data( &im_data ).unwrap(); // Save
}
