// Modified from: https://github.com/ritiek/auto-image-cropper/blob/master/src/main.rs

use image::{
    DynamicImage,
    GenericImageView,
    Rgba,
};

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct ImageCrop {
    pub img: DynamicImage,
}

impl ImageCrop {
    pub fn calculate_corners(&self) -> (Point, Point) {
        (self.top_left_corner(), self.bottom_right_corner())
    }

    fn isnt_rubbish(pixel: Rgba<u8>) -> bool {
        pixel.0[3] != 0
    }

    fn top_left_corner(&self) -> Point {
        Point {
            x: self.top_left_corner_x(),
            y: self.top_left_corner_y(),
        }
    }

    fn top_left_corner_x(&self) -> u32 {
        for x in 0..(self.img.dimensions().0) {
            for y in 0..(self.img.dimensions().1) {
                let pixel = self.img.get_pixel(x, y);
                if Self::isnt_rubbish(pixel) {
                    return x;
                }
            }
        }
        unreachable!();
    }

    fn top_left_corner_y(&self) -> u32 {
        for y in 0..(self.img.dimensions().1) {
            for x in 0..(self.img.dimensions().0) {
                let pixel = self.img.get_pixel(x, y);
                if Self::isnt_rubbish(pixel) {
                    return y;
                }
            }
        }
        unreachable!();
    }

    fn bottom_right_corner(&self) -> Point {
        Point {
            x: self.bottom_right_corner_x(),
            y: self.bottom_right_corner_y(),
        }
    }

    fn bottom_right_corner_x(&self) -> u32 {
        let mut x = self.img.dimensions().0 as i32 - 1;
        // Using while loop as currently there is no reliable built-in
        // way to use custom negative steps when specifying range
        while x >= 0 {
            let mut y = self.img.dimensions().1 as i32 - 1;
            while y >= 0 {
                let pixel = self.img.get_pixel(x as u32, y as u32);
                if Self::isnt_rubbish(pixel) {
                    return x as u32 + 1;
                }
                y -= 1;
            }
            x -= 1;
        }
        unreachable!();
    }

    fn bottom_right_corner_y(&self) -> u32 {
        let mut y = self.img.dimensions().1 as i32 - 1;
        // Using while loop as currently there is no reliable built-in
        // way to use custom negative steps when specifying range
        while y >= 0 {
            let mut x = self.img.dimensions().0 as i32 - 1;
            while x >= 0 {
                let pixel = self.img.get_pixel(x as u32, y as u32);
                if Self::isnt_rubbish(pixel) {
                    return y as u32 + 1;
                }
                x -= 1;
            }
            y -= 1;
        }
        unreachable!();
    }
}
