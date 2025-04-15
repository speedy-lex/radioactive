use std::f64::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use camera::Camera;
use glam::DVec2;
use renderer::Renderer;
use scene::{Scene, Segment};
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;

mod renderer;
pub mod scene;
pub mod camera;

const FPS: usize = 60;

fn main() {
    let sdl_context = sdl3::init().expect("couldn't init sdl3");
    let video_subsystem = sdl_context.video().expect("couldn't init video subystem");

    let mut keys = [false; 512];

    let screen_bounds = video_subsystem.get_primary_display().unwrap().get_bounds().unwrap();
    let window = video_subsystem
        .window("rustray", screen_bounds.width(), screen_bounds.height())
        .position_centered()
        .fullscreen()
        .opengl()
        .build()
        .expect("couldn't build window");

    let mut canvas = window.into_canvas();
    let mut texture_creator = canvas.texture_creator();

    let mut renderer =
        Renderer::new(&mut texture_creator, 600, 400).expect("couldn't init renderer");
    let scene = Scene { segments: vec![
        Segment { a: DVec2::new(1.0, 1.0), b: DVec2::new(-1.0, 1.0), color: Color::RED },
        Segment { a: DVec2::new(-1.0, 1.0), b: DVec2::new(-1.0, -1.0), color: Color::GREEN },
        Segment { a: DVec2::new(-1.0, -1.0), b: DVec2::new(1.0, -1.0), color: Color::BLUE },
        Segment { a: DVec2::new(1.0, -1.0), b: DVec2::new(1.0, 1.0), color: Color::BLACK },
    ] };
    let mut camera = Camera { pos: DVec2::ZERO, rot: 90.0f64.to_radians(), fov: 66.0f64.to_radians() };

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
            camera.pos += 2.0 * vector * dt;
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
