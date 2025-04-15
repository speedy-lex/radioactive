use std::f64::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use camera::Camera;
use glam::DVec2;
use renderer::Renderer;
use scene::{Scene, Segment};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;

mod renderer;
pub mod scene;
pub mod camera;

const FPS: usize = 60;

fn main() {
    let sdl_context = sdl2::init().expect("couldn't init sdl2");
    let video_subsystem = sdl_context.video().expect("couldn't init video subystem");

    let mut keys = [false; 512];

    let window = video_subsystem
        .window("rustray", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .expect("couldn't build window");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut texture_creator = canvas.texture_creator();

    let mut renderer =
        Renderer::new(&mut texture_creator, 400, 300).expect("couldn't init renderer");
    let scene = Scene { segments: vec![Segment { a: DVec2::new(1.0, 3.0), b: DVec2::new(-1.0, 3.0), color: Color::RED }] };
    let mut camera = Camera { pos: DVec2::ZERO, rot: 90.0f64.to_radians(), fov: 60.0f64.to_radians()  };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("couldn't init event pump");

    let mut dt = 0.0;
    'running: loop {
        let start = std::time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { scancode: Some(scan), .. } => {
                    keys[scan as usize] = true
                }
                Event::KeyUp { scancode: Some(scan), .. } => {
                    keys[scan as usize] = false
                }
                _ => {}
            }
        }
        if keys[Scancode::Left as usize] {
            camera.rot -= dt;
        }
        if keys[Scancode::Right as usize] {
            camera.rot += dt;
        }
        if keys[Scancode::W as usize] {
            let vector = DVec2::from_angle(camera.rot);
            camera.pos += vector * dt;
        }
        if keys[Scancode::S as usize] {
            let vector = DVec2::from_angle(PI + camera.rot);
            camera.pos += vector * dt;
        }
        if keys[Scancode::A as usize] {
            let vector = DVec2::from_angle(-FRAC_PI_2 + camera.rot);
            camera.pos += vector * dt;
        }
        if keys[Scancode::D as usize] {
            let vector = DVec2::from_angle(FRAC_PI_2 + camera.rot);
            camera.pos += vector * dt;
        }

        renderer.draw(&scene, &camera);

        canvas.clear();
        renderer.blit(&mut canvas);
        canvas.present();
        let elapsed = start.elapsed().as_secs_f64();
        let to_sleep = (1.0/FPS as f64)-elapsed;
        if to_sleep > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(to_sleep));
        }
        dt = start.elapsed().as_secs_f64();
    }
}
