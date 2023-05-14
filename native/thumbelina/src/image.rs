use rustler::NifStruct;
use image::DynamicImage;

#[derive(NifStruct)]
#[module = "Thumbelina.Image"]
pub struct Image {
    pub extension: String,
    pub height: u32,
    pub width: u32,
}

impl<'a> Image {
    pub fn new(extension: &'a str, image: &DynamicImage) -> Self {
        return Image {
            extension: String::from(extension),
            height: image.height(),
            width: image.width(),
        };
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

// pub struct ImageMetadata {
//     path: String,
//     source: Source,
//     // fn write to dest

//     // fn modify_inplace(extension, path, source, height, width) -> Self {
//     //     thumbelina::Image{e}
//     // }
// }

// pub enum Source {
//     Disk,
//     InMemory,
// }
