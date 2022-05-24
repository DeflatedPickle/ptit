mod imagecrop;

pub mod ptit {
    use bytes::Bytes;
    use image::{DynamicImage, GenericImageView, ImageError, Rgba};
    use image::imageops::FilterType;
    use reqwest;
    use rgb2ansi256::rgb_to_ansi256;
    use termsize::Size;

    use crate::imagecrop::ImageCrop;

    pub fn download(website: &str) -> Result<reqwest::Result<Bytes>, reqwest::Error> {
        return match reqwest::blocking::get(website) {
            Err(e) => Err(e),
            Ok(r) => Ok(r.bytes())
        };
    }

    pub fn load(img_bytes: Bytes) -> Result<DynamicImage, ImageError> {
        return match image::load_from_memory(&img_bytes) {
            Err(e) => Err(e),
            Ok(r) => Ok(r)
        };
    }

    pub fn resize(image: DynamicImage) -> DynamicImage {
        let size = match termsize::get() {
            None => Size { rows: 64, cols: 64 },
            Some(t) => t
        };

        let width = (size.cols as u32 - 1) / 2 ;
        let height = size.rows as u32 - 1;

        return image.resize(
            round(width, 2),
            round(height, 2),
            FilterType::Nearest,
        )
    }

    pub fn crop(image: DynamicImage) -> DynamicImage {
        let mut img = ImageCrop { img: image };
        let (top_left_corner, bottom_right_corner) = img.calculate_corners();

        return img.img.crop(
            top_left_corner.x,
            top_left_corner.y,
            bottom_right_corner.x - top_left_corner.x,
            bottom_right_corner.y - top_left_corner.y,
        );
    }

    pub fn scan(image: DynamicImage, solid: bool) {
        for y in 0..image.height() {
            for x in 0..image.width() {
                convert(image.get_pixel(x, y), solid);
            }
            println!()
        }
    }

    pub fn convert(rgba: Rgba<u8>, solid: bool) {
        #[allow(unreachable_patterns)]
        let char = if solid {
            '█'
        } else {
            match rgba.0[3] {
                0 => ' ',
                1..=64 => '░',
                65..=128 => '▒',
                129..=192 => '▓',
                193..=255 => '█',
                _ => '?'
            }
        };

        for _ in 0..2 {
            if rgba.0[3] > 0 {
                print!(
                    "\x1b[38;5;{:?}m{}\x1b[0m",
                    rgb_to_ansi256(
                        rgba.0[0],
                        rgba.0[1],
                        rgba.0[3],
                    ),
                    char
                );
            } else {
                print!("{}", char)
            }
        }

        // stdout().flush().unwrap();
    }

    fn round(int: u32, multiple: u32) -> u32 {
        return ((int + multiple - 1) / multiple) * multiple;
    }
}