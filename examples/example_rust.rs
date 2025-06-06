use bcore::api::PostProcessPopulation;
use bcore::{PostProcess, PostProcessReader, PostProcessReaderInfo};
use ndarray::s;
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};
fn main() {
    let r_obj = PostProcess::new(
        "exampled",
        Some("./examples/".to_string()));

        if r_obj.is_err()
        {
            println!("Error {:?}",r_obj);
            return;
        }

        let obj  =r_obj.unwrap();


        let x = obj.get_biomass_concentration().unwrap();
        let time: Vec<f64> = obj.time().iter().map(|t| t / 3600.).collect();
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

        plot.write_image("./examples/out_rust.svg", ImageFormat::SVG, 800, 600, 1.0);
        plot.write_html("./examples/out_rust.html");

        println!("{}", plot.to_inline_html(Some("div_plot_x")));

        println!(
            "{}",
            obj.tallies().expect("REASON").to_csv().expect("REASE")
        );
     
}
