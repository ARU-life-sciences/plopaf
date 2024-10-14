use std::path::PathBuf;

use crate::paf::{CigarCoord, CigarCoords};
use anyhow::Result;
use plotters::prelude::*;

// first of all render a simple scatterplot using Vec<CigarCoords>
pub fn plot(coords: Vec<CigarCoords>, out: PathBuf) -> Result<()> {
    let root = BitMapBackend::new(&out, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // get the max x and y values to set the range of the plot
    let max_x = coords
        .iter()
        .map(|c| c.0.iter().map(|cc| cc.x).max().unwrap())
        .max()
        .unwrap();

    let max_y = coords
        .iter()
        .map(|c| c.0.iter().map(|cc| cc.y).max().unwrap())
        .max()
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        // .caption("plopaf", ("Arial", 50).into_font())
        .margin_left(25)
        .margin_bottom(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..max_x as u32, 0u32..max_y as u32)?;

    // we want the numbers to show Kb, Mb, or Gb
    let format_axis_numbers = &|x: &u32| {
        if *x > 1_000_000 {
            format!("{:.1}M", *x as f64 / 1_000_000 as f64)
        } else if *x > 1_000 {
            format!("{:.1}K", *x as f64 / 1_000 as f64)
        } else {
            x.to_string()
        }
    };

    chart
        .configure_mesh()
        .x_label_formatter(format_axis_numbers)
        .y_label_formatter(format_axis_numbers)
        .draw()?;

    for coord in coords {
        chart.draw_series(
            coord
                .0
                .iter()
                .map(|CigarCoord { x, y, .. }| Pixel::new((*x as u32, *y as u32), &BLACK)),
        )?;
    }

    Ok(())
}
