use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use rand::rngs::ThreadRng;
use rand::Rng;
use core::array::from_fn;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let width = 400;
    let height = 400;
    let window = video.window("dots", width, height).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();

    'legs: loop {
        let time = std::time::Instant::now();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'legs,
                Event::MouseButtonDown { .. } => ,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct World<const K: usize, const P: usize> {
    kinds: [Kind<K, P>; K]
}

struct Kind<const K: usize, const P: usize> {
    dots: [Dot; P],
    color: Color,
    radius: f32,
    forces: [f32; K]
}

struct Dot {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn world<const K: usize, const P: usize>() -> World<K, P> {
    let mut rng =
        World {

        }
}

fn update() {
    rand::
}