use image::{DynamicImage};
use gl;

pub struct Texture {
    id: u32
} 

impl Texture {

    pub fn new(texture_path: &str) -> Result<Self, String> {

        let mut img = image::open(texture_path).map_err(|e| e.to_string())?;
        img = img.flipv();
        let format = match img {
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => return Err("Unsupported image format".to_string()),
        };
        let mut texture = 0;
        unsafe { 
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32, 
                img.width() as i32, 
                img.height() as i32, 
                0, 
                format as u32, 
                gl::UNSIGNED_BYTE, 
                img.into_bytes().as_ptr() as *const _ 
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        // return texture;
        Ok(Self { id: texture })
    }
    
    pub fn get_id(&self) -> u32 {
        self.id
    }
}