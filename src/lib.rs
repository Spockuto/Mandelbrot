use std::f64;
use colors_transform::Color;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use colors_transform::{Hsl,Rgb};
use num_complex::Complex;
use web_sys::console;


fn mandlebrot_frame(width : u32, height : u32, iterations : u32, hue : f32, color1 : String) -> Vec<u8>{
    let mut result : Vec<u8> = vec![];

    let mut default :Vec<u8> = vec![];
    let default_color = Rgb::from_hex_str(&color1).unwrap();
    default.push(default_color.get_red() as u8);
    default.push(default_color.get_green() as u8);
    default.push(default_color.get_blue() as u8);
    default.push(255);

    for y in 0..height {
        for x in 0..width{

            let temp_x = (x as f64 * 5_f64 / width as f64) - 2.5_f64;
            let temp_y = (y as f64 * 5_f64 / height as f64) - 2.5_f64;

            let z = Complex::new(temp_x, temp_y);
            let mut cz = z;
            let mut mandlebrot_pixel : bool = false;

            'pixel : for i in 0..iterations {
                cz = cz * cz + z;

                if cz.re * cz.im > 5_f64 {

                    let color_string = Hsl::from(hue, 100_f32, ( (i as f32) / (iterations as f32) * 100.0)  as f32);
                    result.push(color_string.get_red().round() as u8);
                    result.push(color_string.get_green().round() as u8);
                    result.push(color_string.get_blue().round() as u8);
                    result.push(255);
                    mandlebrot_pixel = true;
                    break 'pixel;
                }
            }

            if !mandlebrot_pixel {
                result.extend(&default);
            } 
        }
    }
    return  result;

} 


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn generate_canvas(iterations : u32, hue : u32, color1 : String) -> () {
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

    let mut data = mandlebrot_frame(width as u32, height as u32, iterations, hue as f32, color1);
    console::log_2(&"height : ".into(), &height.into());
    console::log_2(&"width : ".into(), &width.into());
    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width as u32, height as u32).unwrap();
    context.put_image_data(&data, 0.0, 0.0).unwrap();
    let length : JsValue = data.into();
    console::log_2(&"Image_data : ".into(), &length);

}


