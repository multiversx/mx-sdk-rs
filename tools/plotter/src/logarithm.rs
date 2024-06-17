use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=ln(x)"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f32..50f32, -10f32..10f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (0..=500)
            .map(|x| x as f32 / 10f32)
            .map(|x| (x, x.ln())),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
