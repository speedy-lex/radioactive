use sdl2::{pixels::PixelFormatEnum, render::{Texture, TextureAccess, TextureCreator, TextureValueError}};

pub struct Renderer<'a> {
    texture: Texture<'a>,
    width: usize,
    height: usize
}
impl<'a> Renderer<'a> {
    pub fn new<T: 'a>(texture_creator: &mut TextureCreator<T>, width: usize, height: usize) -> Result<Self, TextureValueError> {
        Ok(Self { texture: texture_creator.create_texture(PixelFormatEnum::RGBA32, TextureAccess::Streaming, width as u32, height as u32)?, width, height })
    }
    pub fn draw(&mut self) {
    }
    pub fn blit(&self) {
    }
}