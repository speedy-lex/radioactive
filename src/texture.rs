use bmp::Image;
use glam::{DVec2, Vec3};
use rand::Rng;

#[derive(PartialEq)]
pub enum Texture {
    Solid(Vec3),
    Stretch(Image),
    Repeat(Image),
    Glitch(f64),
    Compound(Box<Texture>, Box<Texture>, BlendMode),
}
impl Texture {
    pub fn sample(&self, uv: DVec2, width: f64, rng: &mut impl Rng) -> Vec3 {
        match self {
            Texture::Solid(vec3) => *vec3,
            Texture::Stretch(image) => {
                let texture_coords = uv * DVec2::new(image.get_width() as f64, image.get_height() as f64);
                let x = texture_coords.x as u32;
                let y = texture_coords.y as u32;
                let color = image.get_pixel(x, y);
                Vec3::new(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0)
            },
            Texture::Repeat(image) => {
                let texture_coords = (DVec2::new(uv.x * width, uv.y) % DVec2::ONE) *  DVec2::new(image.get_width() as f64, image.get_height() as f64);
                let x = texture_coords.x as u32;
                let y = texture_coords.y as u32;
                let color = image.get_pixel(x, y);
                Vec3::new(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0)
            }
            Texture::Compound(a, b, blend) => {
                blend.blend(a.sample(uv, width, rng), b.sample(uv, width, rng))
            }
            Texture::Glitch(amount) => Vec3::splat(rng.random::<f32>().powi(3) * (*amount) as f32)
        }
    }
    pub fn contains_glitch(&self) -> bool {
        match self {
            Texture::Glitch(_) => true,
            Texture::Compound(texture, texture1, _) => texture.contains_glitch() || texture1.contains_glitch(),
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Multiply,
    Add,
    #[default]
    Mean
}
impl BlendMode {
    pub fn blend(&self, a: Vec3, b: Vec3) -> Vec3 {
        match self {
            BlendMode::Multiply => a * b,
            BlendMode::Add => a + b,
            BlendMode::Mean => (a + b) / 2.0,
        }
    }
}