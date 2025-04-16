use glam::{DVec2, DVec3, Vec3};

use crate::{camera::Ray, texture::Texture};

pub struct HitData<'a> {
    pub dist: f64,
    pub point: DVec2,
    pub u: f64,
    pub segment: &'a Segment,
}

pub struct Segment {
    pub a: DVec2,
    pub b: DVec2,
    pub texture: Texture,
}
impl Segment {
    pub fn intersection(&self, r: &Ray) -> Option<(DVec2, f64)> {
        // ax + by + c = 0
        let d = self.b - self.a;
        let (a1, b1, c1) = {
            let normal = DVec2::new(d.y, -d.x);
            let c = -self.a.dot(normal);
            (normal.x, normal.y, c)
        };
        // ax + by + c = 0
        let (a2, b2, c2) = {
            let d = r.dir;
            let normal = DVec2::new(d.y, -d.x);
            let c = -r.origin.dot(normal);
            (normal.x, normal.y, c)
        };

        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Using_homogeneous_coordinates
        let intersection_3d = DVec3::new(a1, b1, c1).cross(DVec3::new(a2, b2, c2));

        // so that we don't divide by zero
        if intersection_3d.z == 0.0 {
            return None
        }

        let intersection = DVec2::new(intersection_3d.x, intersection_3d.y) / intersection_3d.z;
        
        let t = intersection - self.a;
        let t = if d.x.abs() > d.y.abs() {
            t.x / d.x
        } else {
            t.y / d.y
        };
        if !(0.0..=1.0).contains(&t) {
            return None
        }
        Some((intersection, t))
    }
}

pub struct Scene {
    pub segments: Vec<Segment>
}
impl Scene {
    pub fn sample(&self, ray: &Ray) -> Option<HitData> {
        let mut closest_texture = &Segment { a: DVec2::ZERO, b: DVec2::ZERO, texture: Texture::Solid(Vec3::ZERO) };
        let mut closest_u = 0.0;
        let mut closest = DVec2::ZERO;
        let mut dist = f64::INFINITY;

        for segment in &self.segments {
            if let Some((pos, u)) = segment.intersection(ray) {
                let d = (pos - ray.origin) / ray.dir;
                let d = if d.x.is_nan() {
                    d.y
                } else {
                    d.x
                };
                if d > 0.0 && d < dist {
                    closest_texture = segment;
                    closest_u = u;
                    closest = pos;
                    dist = d;
                }
            }
        }

        if dist.is_infinite() {
            return None;
        }
        Some(HitData { dist, point: closest, u: closest_u, segment: closest_texture })
    }
}