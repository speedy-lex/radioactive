use std::f64::consts::{FRAC_PI_2, PI};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use audio::{AudioData, AudioHandler};
use camera::{Camera, Ray};
use glam::DVec2;
use renderer::Renderer;
use scene::{Scene, Segment};
use sdl3::audio::{AudioFormat, AudioSpec};
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Scancode};
use texture::{BlendMode, Texture};

mod renderer;
pub mod scene;
pub mod camera;
pub mod texture;
mod audio;

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

    let mouse = sdl_context.mouse();
    mouse.show_cursor(false);
    mouse.set_relative_mouse_mode(&window, true);

    let audio_data = Arc::new(Mutex::new(AudioData { white_noise: 0.0 }));
    let sound = sdl_context.audio().unwrap();
    let stream = sound.open_playback_stream(&AudioSpec::new(Some(44100), Some(1), Some(AudioFormat::f32_sys())), AudioHandler::new(audio_data.clone(), 44100)).unwrap();
    stream.resume().unwrap();

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();


    let width = 800;
    let height = 600;
    let mut renderer =
        Renderer::new(&texture_creator, width, height).expect("couldn't init renderer");
    let scene = Scene { segments: vec![
        Segment { a: DVec2::new(1000.0, 0.5), b: DVec2::new(-1000.0, 0.5), texture: Texture::Repeat(bmp::open("./brick.bmp").unwrap()) },
        Segment { a: DVec2::new(-1000.0, -0.5), b: DVec2::new(1000.0, -0.5), texture: Texture::Repeat(bmp::open("./brick.bmp").unwrap()) },
        Segment { a: DVec2::new(25.0, -0.5), b: DVec2::new(25.0, 0.5), texture: Texture::Stretch(bmp::open("./brick.bmp").unwrap()) },
        Segment { a: DVec2::new(0.0, -0.5), b: DVec2::new(0.0, 0.5), texture: Texture::Compound(Box::new(Texture::Glitch(0.5)), Box::new(Texture::Stretch(bmp::open("./eyes.bmp").unwrap())), BlendMode::Multiply) },
    ] };
    let mut camera = Camera { pos: DVec2::new(24.5, 0.0), rot: 180.0f64.to_radians(), fov: 66.0f64.to_radians(), noise: 0.0, fog_dist: 1000.0 };

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
                Event::MouseMotion { xrel, .. } => {
                    camera.rot += xrel as f64 / 700.0;
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
        let old = camera.pos;
        if keys[Scancode::W as usize] {
            let vector = DVec2::from_angle(camera.rot);
            let speed = if keys[Scancode::LCtrl as usize] {
                3.0
            } else {
                2.0
            };
            camera.pos += speed * vector * dt;
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
        // undo illegal moves
        // FIXME: stop clipping (implement proper collide and slide)
        {
            let hit = scene.sample(&Ray {
                origin: camera.pos,
                dir: camera.pos - old,
            });
            if let Some(h) = hit {
                if (0.0..=1.1).contains(&h.dist) {
                    camera.pos = old;
                }
            }
        }

        camera.noise = (1.0 - (camera.pos.x.abs() - 1.0).max(0.0) / 10.0).clamp(0.3, 0.995);
        audio_data.lock().unwrap().white_noise = (camera.noise - 0.2) as f32 / 3.0;
        
        {
            let old_width = renderer.width();
            let old_height = renderer.height();
            
            let new_width = (width as f64 / (1.0 + 2.0 * (camera.noise - 0.3).max(0.0))) as usize;
            let new_height = (height as f64 / (1.0 + 2.0 * (camera.noise - 0.3).max(0.0))) as usize;
            
            if new_height.abs_diff(old_height) >= 10 {
                let old = renderer.into_cpu_texture();
                renderer = Renderer::new(&texture_creator, new_width, new_height).unwrap();
                renderer.set_cpu_texture(old, old_width, old_height);
            }
        }
        renderer.draw(&scene, &camera, dt);

        // canvas.clear();
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
