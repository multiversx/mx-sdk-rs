use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub fn draw(canvas: HtmlCanvasElement, max_x: f32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
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
        .build_cartesian_2d(0f32..max_x, -10f32..10f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, x.ln())),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
