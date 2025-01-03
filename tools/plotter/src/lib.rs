#![allow(clippy::type_complexity)]

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub mod plotter_ln_big_float;
pub mod plotter_ln_big_uint;
pub mod plotter_ln_managed_decimal;
mod plotter_log2_managed_decimal;

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
        let map_coord =
            plotter_ln_big_uint::draw_bu_ln(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_uint_error(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            plotter_ln_big_uint::draw_bu_ln_error(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_managed_decimal(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            plotter_ln_managed_decimal::draw_md_ln(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_managed_decimal_error(
        canvas: HtmlCanvasElement,
        max_x: f32,
    ) -> Result<Chart, JsValue> {
        let map_coord = plotter_ln_managed_decimal::draw_md_ln_error(canvas, max_x)
            .map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn log2_managed_decimal(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord = plotter_log2_managed_decimal::draw_md_log2(canvas, max_x)
            .map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn log2_managed_decimal_error(
        canvas: HtmlCanvasElement,
        max_x: f32,
    ) -> Result<Chart, JsValue> {
        let map_coord = plotter_log2_managed_decimal::draw_md_log2_error(canvas, max_x)
            .map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_float(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            plotter_ln_big_float::draw_bf_ln(canvas, max_x).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn ln_big_float_error(canvas: HtmlCanvasElement, max_x: f32) -> Result<Chart, JsValue> {
        let map_coord =
            plotter_ln_big_float::draw_bf_ln_error(canvas, max_x).map_err(|err| err.to_string())?;
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
