use glam::{DVec2, Vec3};
use rand::{distr::{Bernoulli, Distribution}, rng};
use sdl3::{
    pixels::PixelFormat,
    render::{Canvas, RenderTarget, Texture, TextureAccess, TextureCreator, TextureValueError}, sys::pixels::SDL_PIXELFORMAT_RGB96_FLOAT,
};

use crate::{camera::{Camera, Ray}, scene::{HitData, Scene}, texture};

pub struct Renderer<'a> {
    texture: Texture<'a>,
    cpu_texture: Vec<Vec3>,
    width: usize,
    height: usize,
}
impl<'a> Renderer<'a> {
    pub fn new<T: 'a>(
        texture_creator: &'a TextureCreator<T>,
        width: usize,
        height: usize,
    ) -> Result<Self, TextureValueError> {
        let mut x = Self {
            texture: texture_creator.create_texture(
                unsafe { PixelFormat::from_ll(SDL_PIXELFORMAT_RGB96_FLOAT) }, //::from_masks(PixelMasks { bpp: 32, rmask: 0x000000ff, gmask: 0x0000ff00, bmask: 0x00ff0000, amask: 0xff000000 }),
                TextureAccess::Streaming,
                width as u32,
                height as u32,
            )?,
            width,
            height,
            cpu_texture: vec![Vec3::ZERO; width * height],
        };
        x.texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
        Ok(x)
    }
    pub fn draw(&mut self, scene: &Scene, camera: &Camera, dt: f64) {
        for pixel in self.cpu_texture.iter_mut() {
            *pixel *= (-dt).exp() as f32;
        }
        let distribution = Bernoulli::new(camera.noise).unwrap();
        let mut rng = rng();
        for (x, ray) in camera.get_rays(self.width).enumerate() {
            match scene.sample(&ray) {
                Some(HitData {
                    mut dist,
                    point,
                    u,
                    segment,
                }) => {
                    // correct fish eye
                    dist = camera.get_perp_dist_to(point);
                    
                    let projection_distance = self.width as f64 / (2.0 * (camera.fov/2.0).tan());
                    let height = (projection_distance / dist) as usize & (!1);
                    let clamped_height = height.min(self.height); // clamp height to screen
                    let space = (self.height - clamped_height) / 2;

                    let end_y = space + clamped_height;

                    for y in 0..self.height {
                        if (space..end_y).contains(&y) {
                            if segment.texture != texture::Texture::Glitch && distribution.sample(&mut rng) {
                                continue;
                            }
                            let mut color = segment.texture.sample(DVec2::new(u, (y + height/2 - self.height/2) as f64 / height as f64), (segment.b - segment.a).length(), &mut rng);
                            color *= (2.0 / dist).min(1.0) as f32;
                            self.set_pixel(x, y, color);
                        } else {
                            if distribution.sample(&mut rng) {
                                continue;
                            }
                            let color = floor_ceil(y, self.width, self.height, &ray, camera);
                            self.set_pixel(x, y, color);
                        }
                    }
                },
                None => {
                    for y in 0..self.height {
                        if distribution.sample(&mut rng) {
                            continue;
                        }
                        let color = floor_ceil(y, self.width, self.height, &ray, camera);
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
    pub fn set_cpu_texture(&mut self, texture: Vec<Vec3>, old_width: usize, old_height: usize) {
        for (x, color) in self.cpu_texture.iter_mut().enumerate() {
            let y = x / self.width;
            let x = x % self.width;
            
            let u = x as f64 / self.width as f64;
            let v = y as f64 / self.height as f64;

            let a = (u * old_width as f64) as usize;
            let b = (v * old_height as f64) as usize;
            *color = texture[a + b*old_width];
        }
    }
    pub fn into_cpu_texture(self) -> Vec<Vec3> {
        self.cpu_texture
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    /// no bounds check for x
    fn set_pixel(&mut self, x: usize, y: usize, col: Vec3) {
        self.cpu_texture[x + y*self.width] = col.powf(2.2);
    }
}

fn floor_ceil(y: usize, width: usize, height: usize, r: &Ray, camera: &Camera) -> Vec3 {
    let v = if y < height/2 {
        1.0 - y as f64 / height as f64 * 2.0
    } else {
        (y + 1) as f64 / height as f64 * 2.0 - 1.0
    };

    let corrected_dist = 1.0 / v;
    let scale = ((width as f64 / height as f64) / (camera.fov/2.0).tan()) / 2.0;
    let real_dist = corrected_dist / r.dir.project_onto(DVec2::from_angle(camera.rot)).length() * scale;

    let pos  = r.origin + r.dir * real_dist;
    let color = if (pos.x.floor() + pos.y.floor()) % 2.0 == 0.0 {
        Vec3::ZERO
    } else {
        Vec3::splat(0.2)
    };
    color * (2.0 / real_dist).min(1.0) as f32
}
