mod args;
use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView, ImageError };
use std::{ convert::TryInto };

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToDecodeImage(ImageError),
    UnableToFormatImage(String),
    UnableToSaveImage(ImageError)
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buf_cap = height * width * 4;
        let buf = Vec::with_capacity(buf_cap.try_into().unwrap());

        FloatingImage {
            width,
            height,
            data: buf,
            name
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_one, image_one_format) = find_image_from_path(args.image_one)?;
    let (image_two, image_two_format) = find_image_from_path(args.image_two)?;

    if image_one_format != image_two_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_one, image_two) = standardize_size(image_one, image_two);
    let mut output = FloatingImage::new(image_one.width(), image_one.height(), args.output);

    let combined_data = combine_images(image_one, image_two);
    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_one_format) {
        Err(ImageDataErrors::UnableToSaveImage(e))
    } else {
        Ok(())
    }

}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match Reader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e)),
                }
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        },
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

fn get_smallest_dimension(dim_one: (u32, u32), dim_two: (u32, u32)) -> (u32, u32) {
    let pix_one = dim_one.0 * dim_one.1;
    let pix_two = dim_two.0 * dim_two.1;
    return if pix_one < pix_two { dim_one } else { dim_two };
}

fn standardize_size(image_one: DynamicImage, image_two: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_one.dimensions(), image_two.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image_two.dimensions() == (width, height) {
        (image_one.resize_exact(width, height, Triangle), image_two)
    } else {
        (image_one, image_two.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_one: DynamicImage, image_two: DynamicImage) -> Vec<u8> {
    let vec_one = image_one.to_rgba8().into_vec();
    let vec_two = image_two.to_rgba8().into_vec();

    alternate_pixels(vec_one, vec_two)
}

fn alternate_pixels(vec_one: Vec<u8>, vec_two: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_one.len()];

    let mut i = 0;
    while i < vec_one.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_one, i, i+3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_two, i, i + 3));
        }
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bounds")
        };
        rgba.push(val);
    }
    rgba
}