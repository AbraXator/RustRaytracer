use std::{fs::File, io::Write};

use crate::core::color::Color;

pub struct PPM<'a> {
    format: u8,
    width: usize,
    height: usize,
    max_color_value: usize,
    bitmap: &'a [Color],
}

impl PPM<'_> {
    pub fn new(width: usize, height: usize, bitmap: &Vec<Color>) -> PPM {
        PPM {
            format: 6, //PPM File format; 3 for ASCII (eg. 0-255), 6 for binary
            width,
            height,
            max_color_value: 255,
            bitmap,
        }
    }

    //Creates and writes a new PPM file
    pub fn write(&self, filename: &str) {
        let mut file = &File::create(filename).unwrap();
        let header = format!(
            "P{}\n{} {}\n{}\n",
            self.format, self.width, self.height, self.max_color_value
        );
        write!(file, "{}", header).unwrap();

        let mut buffer: Vec<u8> = Vec::with_capacity(self.width * self.width * 3);

        for i in 0..self.height {
            for j in 0..self.width {
                let c = self.bitmap[i * self.width + j];
                let c = c.u8();

                buffer.push(c.0);
                buffer.push(c.1);
                buffer.push(c.2);
            }
        }

        file.write_all(buffer.as_slice()).unwrap();
        file.flush().unwrap();
    }
}
