use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use core::array;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_FORCE: f32 = 1.0;
const MAX_RADIUS: f32 = 100.0;
const VISCOSITY: f32 = 0.8;
const POPULATION: usize = 1024;
const VARIANTS: usize = 4;
const COLORS: [Color; VARIANTS] = [
    Color::RED,
    Color::CYAN,
    Color::GREEN,
    Color::MAGENTA
];

struct Dot {
    kind: u8,

}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video.window("dots", WIDTH, HEIGHT).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut rng = fastrand::Rng::new();
    let radii: [f32; VARIANTS] = array::from_fn(|_| rng.f32() * MAX_RADIUS);
    let forces: [f32; VARIANTS * VARIANTS] = array::from_fn(|_| (rng.f32() - 0.5) * MAX_FORCE / 0.5);
    let mut dot: *mut Dot = array::from_fn(|_| Dot {
        x: rng.f32() * WIDTH as f32, y: rng.f32() * HEIGHT as f32, vx: 0.0, vy: 0.0
    }).as_mut_ptr(); //::<_, {VARIANTS * POPULATION}, _>
    'legs: loop {
        let start = Instant::now();
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event { break 'legs }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let mut ptr = dot.add();
        for variant in 0..VARIANTS {
            let radius = radii[variant];
            for offset1 in 0..POPULATION {
                let dot = unsafe {
                    dot = dot.add(1);
                    dot.read()
                };

                for offset2 in offset1 + 1..POPULATION {
                    let other = unsafe { dots.add(offset2).read() };
                    let dx = dot.x - other.x;
                    let dy = dot.y - other.y;
                    let d = dx * dx + dy * dy;
                    if d < radius {
                        dot.vx += dx / d;
                        dot.vy += dy / d;
                        other.vx -= dx / d;
                        other.vy -= dy / d;
                    }
                }

                dot.vx *= VISCOSITY;
                dot.x += dot.vx;
                if dot.x < 0.0 || dot.x >= WIDTH as f32 { dot.x -= dot.vx * 2.0 }
                dot.vy *= VISCOSITY;
                dot.y += dot.vy;
                if dot.y < 0.0 || dot.y >= WIDTH as f32 { dot.y -= dot.vy * 2.0 }
                points[a] = Point::new(dot.x as i32, dot.y as i32);
            }
        }
        canvas.set_draw_color(COLORS[variant]);
        canvas.draw_points(points.as_slice()).unwrap();
        canvas.present();
        let end = Instant::now() - start;
        thread::sleep(end.max(Duration::from_nanos(1_000_000_000 / 60)));
    }
}
