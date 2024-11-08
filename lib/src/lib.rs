#[allow(warnings)]
mod bindings;

use bindings::exports::component::image_manipulation_lib::image_manipulation::{
    Guest, Image, ImageError,
};
use data_encoding::HEXUPPER;
use sha2::Digest;
use sha2::Sha256;

struct Component;
impl Guest for Component {
    fn grayscale(img: Image, quality: u8) -> Result<Image, ImageError> {
        println!(
            "[image-manipulation]: Executing grayscale transform for input image: {}",
            hash(&img)
        );

        let mut i = photon_rs::native::open_image_from_bytes(&img)?;
        photon_rs::monochrome::grayscale(&mut i);

        Ok(i.get_bytes_jpeg(quality))
    }

    fn sepia(img: Image, quality: u8) -> Result<Image, ImageError> {
        println!(
            "[image-manipulation]: Executing sepia transform for input image: {}",
            hash(&img)
        );
        let mut i = photon_rs::native::open_image_from_bytes(&img)?;
        photon_rs::monochrome::sepia(&mut i);

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

fn hash(img: &Image) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&img);
    let result = hasher.finalize();

    HEXUPPER.encode(&result).to_string()
}

bindings::export!(Component with_types_in bindings);
