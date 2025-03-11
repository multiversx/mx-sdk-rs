use crate::DrawResult;
use multiversx_sc::{
    types::{LnDecimals, ManagedDecimalSigned},
};
use multiversx_sc_scenario::api::StaticApi;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub fn draw_md_log2(
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
        .caption(format!("y=log2(x), x=1..{max_x}"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f32..max_x, -5f32..5f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, log2_baseline(x))),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, managed_decimal_log2(x))),
        &GREEN,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

pub fn draw_md_log2_error(
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
        .caption(format!("y=logarithm error, x=1..{max_x}"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(50u32)
        .build_cartesian_2d(0f32..max_x, -0.0001f32..0.0001f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, managed_decimal_log2(x) - log2_baseline(x))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

fn managed_decimal_log2(x: f32) -> f32 {
    let dec = ManagedDecimalSigned::<StaticApi, LnDecimals>::from(x);
    if let Some(ln_dec) = dec.log2() {
        ln_dec.to_big_float().to_f64() as f32
    } else {
        0.0
    }
}

fn log2_baseline(x: f32) -> f32 {
    x.log2()
}

#[cfg(test)]
mod test {
    #[test]
    fn sc_log2_test() {
        assert_eq!(super::managed_decimal_log2(0.0), 0.0);
        println!("{}", super::managed_decimal_log2(1.0));
        println!("{}", super::managed_decimal_log2(2.0));
        println!("{}", super::managed_decimal_log2(3.0));
        println!("{}", super::managed_decimal_log2(4.0));
        println!("{}", super::managed_decimal_log2(5.0));
        println!("{}", super::managed_decimal_log2(200.0));
        // assert!(super::big_uint_ln(1.0) >= 0.0);
        // assert!(super::big_uint_ln(1.0) < 0.01);
        // assert!(super::big_uint_ln(2.0) > 0.6);
    }
}
