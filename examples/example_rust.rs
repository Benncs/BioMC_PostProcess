use bcore::PostProcess;
use ndarray::s;
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};
fn main() {
    if let Ok(obj) = PostProcess::new(
        "cstr",
        Some("/home-local/casale/Documents/thesis/simulations/ecoli_model_2024/out".to_string()),
    ) {
        let x = obj.get_biomass_concentration();
        let time: Vec<f64> = obj.time().into_iter().map(|t| t / 3600.).collect();
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

        plot.write_image("./examples/out.png", ImageFormat::PNG, 800, 600, 1.0);
    }
}
