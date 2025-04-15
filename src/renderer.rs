use sdl3::{
    pixels::{Color, PixelFormat, PixelMasks},
    render::{Canvas, RenderTarget, Texture, TextureAccess, TextureCreator, TextureValueError},
};

use crate::{camera::Camera, scene::{HitData, Scene}};

pub struct Renderer<'a> {
    texture: Texture<'a>,
    cpu_texture: Vec<Color>,
    width: usize,
    height: usize,
}
impl<'a> Renderer<'a> {
    pub fn new<T: 'a>(
        texture_creator: &'a mut TextureCreator<T>,
        width: usize,
        height: usize,
    ) -> Result<Self, TextureValueError> {
        let mut x = Self {
            texture: texture_creator.create_texture(
                PixelFormat::from_masks(PixelMasks { bpp: 32, rmask: 0x000000ff, gmask: 0x0000ff00, bmask: 0x00ff0000, amask: 0xff000000 }),
                TextureAccess::Streaming,
                width as u32,
                height as u32,
            )?,
            width,
            height,
            cpu_texture: vec![Color::BLUE; width * height],
        };
        x.texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        Ok(x)
    }
    pub fn draw(&mut self, scene: &Scene, camera: &Camera) {
        for (x, ray) in camera.get_rays(self.width).enumerate() {
            match scene.sample(&ray) {
                Some(HitData {
                    mut dist,
                    point,
                    color,
                }) => {
                    // correct fish eye
                    // dist *= (ray.dir.to_angle() - camera.rot).cos();
                    dist = camera.get_perp_dist_to(point);
                    
                    let projection_distance = self.width as f64 / (2.0 * (camera.fov/2.0).tan());
                    let mut height = (projection_distance / dist) as usize & (!1);
                    height = height.min(self.height); // clamp height to screen
                    let space = (self.height - height) / 2;

                    let end_y = space + height;

                    for y in 0..self.height {
                        if (space..end_y).contains(&y) {
                            self.set_pixel(x, y, color);
                        } else {
                            let color = lerp(Color::RGB(127, 179, 255), Color::RGB(255, 255, 255), y as f64 / self.height as f64);
                            self.set_pixel(x, y, color);
                        }
                    }
                },
                None => {
                    for y in 0..self.height {
                        let color = lerp(Color::RGB(127, 179, 255), Color::RGB(255, 255, 255), y as f64 / self.height as f64);
                        self.set_pixel(x, y, color);
                    }
                },
            }
        }

        self.texture.with_lock(None, |x, y| {
            for (bytes, colors) in x.chunks_mut(y).zip(self.cpu_texture.chunks(self.width)) {
                bytes[..std::mem::size_of_val(colors)].copy_from_slice(unsafe {
                    std::slice::from_raw_parts(colors.as_ptr().cast(), std::mem::size_of_val(colors))
                });
            }
        }).expect("texture error");
    }
    pub fn blit(&self, canvas: &mut Canvas<impl RenderTarget>) {
        canvas.copy(&self.texture, None, None).unwrap();
    }

    /// no bounds check for x
    fn set_pixel(&mut self, x: usize, y: usize, col: Color) {
        self.cpu_texture[x + y*self.width] = col;
    }
}

fn lerp(a: Color, b: Color, t: f64) -> Color {
    Color { r: lerp_u8(a.r, b.r, t), g: lerp_u8(a.g, b.g, t), b: lerp_u8(a.b, b.b, t), a: lerp_u8(a.a, b.a, t) }
}
fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 * (1.0 - t) + b as f64 * t) as u8
}