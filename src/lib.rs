use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// neat macro wrapper for js console.log 
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// struct for complex numbers
struct Complex {
    real: f64,
    imag: f64,
}

fn build_complex(real: f64, imag: f64) -> Complex {
    Complex {
        real,
        imag,
    }
}

// methods for complex numbers
impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        build_complex(self.real + other.real, self.imag + other.imag)
    }

    fn mag(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    fn mul(&self, other: &Complex) -> Complex {
        let real_part = self.real*other.real - self.imag*other.imag;
        let imag_part = self.imag*other.real + self.real*other.imag;
        build_complex(real_part, imag_part)
    }
}

// refactor 
fn in_mandelbrot(cplx: &Complex) -> bool {
    const ITER_CONST: i32 = 100;
    let mut z = build_complex(0.0, 0.0);

    let mut in_set = false;

    // reverse mandelbrot lul
  /*  for count in 0..ITER_CONST {

        z = (z.mul(&z)).add(&cplx); // z = z^2 + cplx
        if z.mag() > 2.0 {
            in_set = true;
            break;
        }
    } */

    let mut count = 0;

    while z.mag() <= 2.0 {
        z = (z.mul(&z)).add(&cplx); // z = z^2 + cplx
        count += 1;
        
        if count >= ITER_CONST {
            in_set = true;
            break;
        }
    }

    in_set
}

fn initialize_points(x_len: i32, y_len: i32) -> Vec<Vec<bool>> {
    vec![vec![false; x_len as usize] ; y_len as usize]
}

fn fill_mandelbrot(points_array: &mut Vec<Vec<bool>>) {
    let x_len: usize = points_array[0].len();
    let y_len: usize = points_array.len();

    let x_step: f64 = 4.0/(x_len as f64);
    let y_step: f64 = 4.0/(y_len as f64);

    // initial values
    let mut x: f64 = -2.0;
    let mut y: f64 = -2.0;

    for count_y in 0..y_len {
        for count_x in 0..x_len {
            points_array[count_y as usize][count_x as usize] = in_mandelbrot(&build_complex(x,y));
            x += x_step;
        }
        y += y_step;
        x = -2.0;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let y_len: i32 = canvas.height() as i32;
    let x_len: i32 = canvas.width() as i32;

    let mut points_array = initialize_points(x_len, y_len);
    fill_mandelbrot(& mut points_array);

    for y in 0..y_len as usize {
        for x in 0..x_len as usize {
            if points_array[x][y] {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
