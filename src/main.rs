use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod renderer;

fn main() {
    let sdl_context = sdl2::init().expect("couldn't init sdl2");
    let video_subsystem = sdl_context.video().expect("couldn't init video subystem");

    let window = video_subsystem
        .window("rustray", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("couldn't build window");

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("couldn't init event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(texture, None, None).unwrap();
        canvas.present();
    }
}