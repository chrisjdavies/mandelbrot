use gfx_demo;
use num::{Complex, Zero};
use num::complex::ComplexFloat;

const TITLE: &'static str = "Mandelbrot";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const CANVAS_WIDTH: usize = WINDOW_WIDTH / 2;
const CANVAS_HEIGHT: usize = WINDOW_HEIGHT / 2;
const X_MIN: f64 = -2.;
const X_MAX: f64 = 1.;
const Y_MIN: f64 = -1.2;
const Y_MAX: f64 = 1.2;
const ITER_MAX: u32 = 30;

fn main() {
    let mut row = 0;

    gfx_demo::gfx_demo(
        TITLE,
        WINDOW_WIDTH, WINDOW_HEIGHT,
        CANVAS_WIDTH, CANVAS_HEIGHT,
        10,
        |pixels| {
            if row < CANVAS_HEIGHT {
                let y = Y_MIN + ((row as f64 / CANVAS_HEIGHT as f64) * (Y_MAX - Y_MIN));

                for col in 0..CANVAS_WIDTH {
                    let x = X_MIN + ((col as f64 / CANVAS_WIDTH as f64) * (X_MAX - X_MIN));
                    let c = Complex::new(x, y);
                    let mut z: Complex<f64> = Complex::zero();
                    let mut iters = 0;

                    while iters < ITER_MAX && z.abs() < 2.0f64 {
                        z = z.powi(2) + c;
                        iters += 1;
                    }

                    let intensity = 255 - ((iters * 255) / ITER_MAX);
                    pixels[(row * CANVAS_WIDTH) + col] = 0xff000000u32 | intensity;
                }

                row += 1;
            }
        }
    ).unwrap();
}
