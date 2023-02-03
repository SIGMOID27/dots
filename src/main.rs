use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Instant;
use std::time::Duration;
use std::thread;
use core::array;
use std::ops::Range;
use rand::Rng;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const IX: Range<f32> = 0.0..WIDTH as f32;
const IY: Range<f32> = 0.0..HEIGHT as f32;
const FORCES: Range<f32> = -1.0..1.0;
const RADII: Range<f32> = 20.0..80.0;
const VISCOSITY: f32 = 0.7;
const POPULATION: usize = 100;
const COLORS: [Color; 4] = [
    Color::RED,
    Color::CYAN,
    Color::GREEN,
    Color::MAGENTA
];

struct Dots {
    color: Color,
    radius: f32,
    forces: [f32; COLORS.len()],
    dots: [Dot; POPULATION],
}

struct Dot {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video.window("dots", WIDTH, HEIGHT).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut world = COLORS.map(|color| Dots {
        color,
        radius: rng.gen_range(RADII),
        forces: array::from_fn(|_| rng.gen_range(FORCES)),
        dots: array::from_fn(|_| Dot {
            x: rng.gen_range(IX),
            y: 0.0,
            vx: rng.gen_range(IY),
            vy: 0.0
        })
    });
    'legs: loop {
        let start = Instant::now();
        for event in events.poll_iter() {
            if let Event::Quit {..} = event { break 'legs }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.present();
        let end = Instant::now() - start;
        thread::sleep(end.max(Duration::from_nanos(1_000_000_000 / 60)));
    }
}
