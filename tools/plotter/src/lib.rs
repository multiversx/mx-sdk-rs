#![allow(clippy::type_complexity)]

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub mod logarithm_bf;
pub mod logarithm_bu;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw provided functions on the canvas.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn ln_big_uint(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord = logarithm_bu::draw_bu_logs(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_uint_error(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            logarithm_bu::draw_bu_error(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_float(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord = logarithm_bf::draw_bf_logs(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_float_error(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            logarithm_bf::draw_bf_error(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
