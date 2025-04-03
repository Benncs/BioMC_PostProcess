use bcore::{PostProcess, PostProcessReader};
use ndarray::s;
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};
fn main() {
    if let Ok(obj) = PostProcess::new(
        "str_1",
        Some("/home-local/casale/Documents/thesis/simulations/meta/out_str_nu_redis_2".to_string()),
    ) {
        let x = obj.get_biomass_concentration().unwrap();
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

        plot.write_image("./examples/out.svg", ImageFormat::SVG, 800, 600, 1.0);
        plot.write_html("./examples/out.html");

        println!("{}", plot.to_inline_html(Some("div_plot_x")));

        println!(
            "{}",
            obj.tailles().expect("REASON").to_csv().expect("REASE")
        );
    } else {
        println!("Simulation not found");
    }
}
