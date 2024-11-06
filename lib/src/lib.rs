#[allow(warnings)]
mod bindings;

use bindings::exports::component::image_manipulation_lib::image_manipulation::{
    Guest, Image, ImageError,
};

struct Component;
impl Guest for Component {
    fn grayscale(img: Image, quality: u8) -> Result<Image, ImageError> {
        let mut i = photon_rs::native::open_image_from_bytes(&img)?;
        photon_rs::monochrome::grayscale(&mut i);

        Ok(i.get_bytes_jpeg(quality))
    }
}

impl From<photon_rs::native::Error> for ImageError {
    fn from(e: photon_rs::native::Error) -> Self {
        match e {
            photon_rs::native::Error::ImageError(image_error) => {
                Self::ImageError(image_error.to_string())
            }
            photon_rs::native::Error::IoError(error) => Self::IoError(error.to_string()),
        }
    }
}

bindings::export!(Component with_types_in bindings);
