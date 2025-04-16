use glam::DVec2;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: DVec2,
    pub dir: DVec2,
}

pub struct Camera {
    pub pos: DVec2,
    pub rot: f64,
    pub fov: f64,
    pub noise: f64,
}
impl Camera {
    pub fn get_rays(&self, n: usize) -> impl Iterator<Item = Ray> {
        (0..n).map(move |x| {
            let r = x as f64 / (n-1) as f64;
            let theta = self.rot + self.fov * (r - 0.5);
            Ray {
                dir: DVec2::from_angle(theta),
                origin: self.pos,
            }
        })
    }
    pub fn get_perp_dist_to(&self, pos: DVec2) -> f64 {
        (pos - self.pos).project_onto(DVec2::from_angle(self.rot)).length()
    }
}
