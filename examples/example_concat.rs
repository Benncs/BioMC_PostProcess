use bcore::{ConcatPostPrcess,PostProcessReader};
use ndarray::s;
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};
fn main() {
    if let Ok(obj) = ConcatPostPrcess::new(
        &["uptake",
    "uptake_22",
    "uptake_23",
    "uptake_24",
    "uptake_25",
    "uptake_26",
    "uptake_27",
    "uptake_28",
    "uptake_29"],
        Some("/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/".to_string()),
    ) {
        println!("{:?}",obj.get_property_names());
        let x = obj.get_biomass_concentration().unwrap();
        let time: Vec<f64> = obj.time_array().into_iter().map(|t| t / 3600.).collect();
        let xvec = x.slice(s![.., 0]).to_vec();

        let mut plot = Plot::new();
        let trace = Scatter::new(time, xvec)
            .name("Biomass Concentration")
            .mode(plotly::common::Mode::Lines)
            .line(plotly::common::Line::new().color("red").width(2.));

        plot.add_trace(trace);

        let layout = Layout::new()
            .title("Biomass Concentration Over Time")
            .x_axis(plotly::layout::Axis::new().title("Time (h)"))
            .y_axis(plotly::layout::Axis::new().title("Biomass Concentration (g/L)"));

        plot.set_layout(layout);

        plot.write_image("./examples/out_concat.svg", ImageFormat::SVG, 800, 600, 1.0);
        plot.write_html("./examples/out_concat.html");
        plot.show();

        println!("{}",plot.to_inline_html(Some("div_plot_x")));
    }
    else {
        println!("Simulation not found");
    }
}
