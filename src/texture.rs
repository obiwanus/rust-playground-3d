use gl::types::*;
use stb_image::image::{self, LoadResult};

#[derive(Debug, Fail)]
pub enum TextureError {
    #[fail(display = "Image format F32 is not supported")]
    FormatNotSupported,
    #[fail(display = "Cannot load texture image: {}", msg)]
    LoadError { msg: String },
}

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture { id }
    }

    pub fn bind(&self, unit: i32) {
        unsafe {
            gl::ActiveTexture(Texture::unit_to_gl_const(unit));
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn unit_to_gl_const(unit: i32) -> GLenum {
        match unit {
            0 => gl::TEXTURE0,
            1 => gl::TEXTURE1,
            2 => gl::TEXTURE2,
            3 => gl::TEXTURE3,
            4 => gl::TEXTURE4,
            5 => gl::TEXTURE5,
            6 => gl::TEXTURE6,
            7 => gl::TEXTURE7,
            8 => gl::TEXTURE8,
            9 => gl::TEXTURE9,
            10 => gl::TEXTURE10,
            11 => gl::TEXTURE11,
            12 => gl::TEXTURE12,
            13 => gl::TEXTURE13,
            14 => gl::TEXTURE14,
            15 => gl::TEXTURE15,
            _ => panic!("Unsupported texture unit"),
        }
    }

    pub fn set_default_parameters(self) -> Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        }
        self
    }

    pub fn load_image(self, path: &str) -> Result<Self, TextureError> {
        unsafe {
            stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
        }

        // Load image from disk
        let img = match image::load_with_depth(path, 3, false) {
            LoadResult::ImageU8(image) => Ok(image),
            LoadResult::ImageF32(_) => Err(TextureError::FormatNotSupported),
            LoadResult::Error(msg) => Err(TextureError::LoadError { msg }),
        }?;

        // Send pixels to GPU
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                img.width as GLint,
                img.height as GLint,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(self)
    }
}
