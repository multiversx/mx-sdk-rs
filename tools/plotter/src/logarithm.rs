use crate::DrawResult;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::StaticApi;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub fn draw(
    canvas: HtmlCanvasElement,
    max_x: f32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let root = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();

    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=ln(x), x=1..{max_x}"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f32..max_x, -1f32..5f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, x.ln())),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, sc_ln(x))),
        &GREEN,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

pub fn sc_ln(x: f32) -> f32 {
    let bu = BigUint::<StaticApi>::from(x as u32);
    if let Some(ln_dec) = bu.ln() {
        let ln_units = ln_dec.into_raw_units().to_u64().unwrap();
        let ln_sf = ln_dec.scaling_factor().to_u64().unwrap();
        (ln_units as f64 / ln_sf as f64) as f32
    } else {
        0.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn sc_ln_test() {
        assert_eq!(super::sc_ln(0.0), 0.0);
        assert!(super::sc_ln(1.0) > 0.0);
        assert!(super::sc_ln(1.0) < 0.01);
        assert!(super::sc_ln(2.0) > 0.6);
    }
}
