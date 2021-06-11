use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Uint8ClampedArray, WebAssembly};

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
    fn add(mut self, other: &Complex) -> Complex {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        self.real = real;
        self.imag = imag;
        self
    }

    fn mag(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    fn square(mut self) -> Complex {
        let real_part = self.real*self.real - self.imag*self.imag;
        let imag_part = self.imag*self.real + self.real*self.imag;
        self.real = real_part;
        self.imag = imag_part;
        self
    }

}

// refactor 
fn in_mandelbrot(cplx: &Complex) -> bool {
    const ITER_CONST: i32 = 100;
    let mut z = build_complex(0.0, 0.0);

    let mut in_set = false;
    let mut count = 0;

    while z.mag() <= 2.0 {
        z = (z.square()).add(&cplx); // z = z^2 + cplx
        count += 1;

        if count >= ITER_CONST {
            in_set = true;
            break;
        }
    }

    in_set
}


fn fill_mandelbrot(points_array: & mut Vec<u8>, x_len: i32, y_len: i32) {

    let len = points_array.len();
    let num_pixels = len/4;

    let x_step = 4.0/(x_len as f64);
    let y_step = 4.0/(y_len as f64);

    for count in 0..num_pixels {
        let c = count as i32;

        let x = c % x_len;
        let y = c / x_len;

        let x_com = -2.0 + (x as f64)* x_step;
        let y_com = -2.0 + (y as f64) * y_step;

        let in_set = in_mandelbrot(&build_complex(x_com, y_com));

        if in_set {
            points_array[count*4 + 3] = 255;
        }
    }

}

// lets us make an ImageData
#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor, catch)]
    fn new(data: &Uint8ClampedArray, width: f64, height: f64) -> Result<ImageData, JsValue>;
}

#[wasm_bindgen]
pub fn run() -> ImageData{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let y_len: i32 = canvas.height() as i32;
    let x_len: i32 = canvas.width() as i32;

    let pixels: usize = (x_len * y_len) as usize;

    let mut points_array = vec![0; pixels*4];
    fill_mandelbrot(&mut points_array, x_len, y_len);

    let pointer = points_array.as_ptr() as usize;

    let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
    let new_array = Uint8ClampedArray::new(&mem.buffer()).slice(pointer as u32, (pointer + pixels*4) as u32);
    ImageData::new(&new_array, x_len.into(), y_len.into()).unwrap()

}
