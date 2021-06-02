use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

struct Complex {
    real: f64,
    imag: f64,
}

// struct for complex numbers
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


fn mandelbrot(cplx: &Complex) -> bool {
    const ITER_CONST: i32 = 100;
    let mut z = build_complex(0.0, 0.0);

    let mut in_set = false;
    for count in 0..ITER_CONST {

        z = (z.mul(&z)).add(&cplx); // z = z^2 + cplx
        if z.mag() > 2.0 {
            in_set = true;
            break;
        }
    }
    in_set
}

fn initialize_points(x_len: i32, y_len: i32) -> Vec<Vec<bool>> {
    let points_array = vec![vec![false; x_len as usize] ; y_len as usize];
    points_array
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

    let height: i32 = canvas.height() as i32;
    let width: i32 = canvas.width() as i32;
    let x_step: f64 = 4.0/(width as f64);
    let y_step: f64 = 4.0/(height as f64);

    //log!("{}", canvas.height());

    // initial values
    let mut x: f64 = -2.0;
    let mut y: f64 = -2.0;

    let mut points_array = vec![vec![false; width as usize] ; height as usize];

    for count_y in 0..height {
        for count_x in 0..width {
            points_array[count_y as usize][count_x as usize] = mandelbrot(&build_complex(x,y));
            x += x_step;
        }
        y += y_step;
        x = -2.0;
    }

    let ylen = points_array.len();
    let xlen = points_array[0].len();

    for y in 0..ylen {
        for x in 0..xlen {
            if !points_array[x][y] {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
