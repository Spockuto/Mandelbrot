use colors_transform::Color;
use colors_transform::{Hsl, Rgb};
use num_complex::Complex;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

fn mandelbrot_zoom_frame(
    width: u32,
    height: u32,
    iterations: u32,
    hue: f32,
    color: Vec<u8>,
    zoom: f64,
    center_x: f64,
    center_y: f64,
) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let scale_x = 5.0 / width as f64 / zoom;
    let scale_y = 5.0 / height as f64 / zoom;

    for y in 0..height {
        for x in 0..width {
            let temp_x = (x as f64 * scale_x) - 2.5 / zoom + center_x;
            let temp_y = (y as f64 * scale_y) - 2.5 / zoom + center_y;

            let z = Complex::new(temp_x, temp_y);
            let mut cz = z;
            let mut mandelbrot_pixel: bool = false;

            'pixel: for i in 0..iterations {
                cz = cz * cz + z;

                if cz.norm_sqr() > 5.0 {
                    let color_string = Hsl::from(
                        hue,
                        100_f32,
                        ((i as f32) / (iterations as f32) * 100.0) as f32,
                    );
                    result.push(color_string.get_red().round() as u8);
                    result.push(color_string.get_green().round() as u8);
                    result.push(color_string.get_blue().round() as u8);
                    result.push(255);
                    mandelbrot_pixel = true;
                    break 'pixel;
                }
            }

            if !mandelbrot_pixel {
                result.extend(&color);
            }
        }
    }
    return result;
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn generate_canvas(center: u32, frames: u32, iterations: u32, hue: u32, color: String) -> () {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let canvas_div = document
        .get_element_by_id("canvasdiv")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let height = canvas_div.client_height();
    let width = canvas_div.client_width();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let (x, y) = match center {
        1 => (-0.75, 0.1),
        2 => (-2.0, 0.0),
        3 => (-0.10109636384562, 0.95628651080914),
        4 => (-0.77568377, 0.13646737),
        _ => (1.0, 1.0),
    };

    let mut default: Vec<u8> = vec![];
    let default_color = Rgb::from_hex_str(&color).unwrap();
    default.push(default_color.get_red() as u8);
    default.push(default_color.get_green() as u8);
    default.push(default_color.get_blue() as u8);
    default.push(255);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut zoom = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if zoom > frames {
            let _ = f.borrow_mut().take();
            return;
        }

        zoom += 1;
        let mut data = mandelbrot_zoom_frame(
            width as u32,
            height as u32,
            iterations,
            hue as f32,
            default.clone(),
            zoom as f64,
            x.clone(),
            y.clone(),
        );
        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            width as u32,
            height as u32,
        )
        .unwrap();
        context.put_image_data(&data, 0.0, 0.0).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
