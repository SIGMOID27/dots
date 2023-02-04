use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Instant;
use std::time::Duration;
use std::thread;
use core::array;
use std::ops::Range;
use rand::Rng;
use sdl2::rect::Point;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const FORCES: Range<f32> = -1.0..1.0;
const RADII: Range<f32> = 20.0..80.0;
const VISCOSITY: f32 = 0.7;
const POPULATION: usize = 100;
const VARIANTS: usize = 4;
const COLORS: [Color; VARIANTS] = [
    Color::RED,
    Color::CYAN,
    Color::GREEN,
    Color::MAGENTA
];

struct Dots {
    radius: f32,
    forces: [f32; VARIANTS],
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
    let window = video.window("dots", WIDTH as u32, HEIGHT as u32).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut world: [Dots; VARIANTS] = array::from_fn(|_| Dots {
        radius: rng.gen_range(RADII),
        forces: array::from_fn(|_| rng.gen_range(FORCES)),
        dots: array::from_fn(|_| Dot {
            x: rng.gen_range(0.0..WIDTH),
            y: rng.gen_range(0.0..HEIGHT),
            vx: 0.0,
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
        for i in 0..VARIANTS {
            let world_split = world.split_at_mut(i + 1);
            let dots = &mut world_split.0[i];
            let force = dots.forces[i];
            let mut points = [Point::new(0, 0); POPULATION];
            for j in 0..POPULATION {
                let dots_split = dots.dots.split_at_mut(j + 1);
                let dot = &mut dots_split.0[j];
                for other in dots_split.1 {
                    let dx = dot.x - other.x;
                    let dy = dot.y - other.y;
                    let d = dx * dx + dy * dy;
                    if d < dots.radius {
                        dot.vx += dx / d;
                        dot.vy += dy / d;
                    }
                }
                for other_dots in world_split.1.iter_mut() {
                    let other_force = other_dots.forces[i];
                    for other in &mut other_dots.dots {

                    }
                }
                dot.vx *= VISCOSITY;
                dot.x += dot.vx;
                if dot.x < 0.0 || dot.x >= WIDTH { dot.x -= dot.vx * 2.0 }

                dot.vy *= VISCOSITY;
                dot.y += dot.vy;
                if dot.y < 0.0 || dot.y >= WIDTH { dot.y -= dot.vy * 2.0 }

                points[j] = Point::new(dot.x as i32, dot.y as i32);
            }
            canvas.set_draw_color(COLORS[i]);
            canvas.draw_points(points.as_slice()).unwrap();
        }
        canvas.present();
        let end = Instant::now() - start;
        thread::sleep(end.max(Duration::from_nanos(1_000_000_000 / 60)));
    }
}
