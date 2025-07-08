use biomc_pp_core::api::PostProcessPopulation;
use biomc_pp_core::{PostProcess, PostProcessReader, PostProcessReaderInfo};
use ndarray::s;
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};
fn main() {
    let r_obj = PostProcess::new(
        "example0d",
        Some("./examples/".to_string()));

   

    if r_obj.is_err() {
        println!("Error {:?}", r_obj);
        return;
    }

    let obj = r_obj.unwrap();

    let x = obj.get_biomass_concentration().unwrap();
    let s = obj.get_spatial_average_concentration(0, biomc_pp_core::api::Phase::Liquid);
    let time: Vec<f64> = obj.time().iter().map(|t| t / 3600.).collect();
    let xvec = x.slice(s![.., 0]).to_vec();
    let mut plot = Plot::new();
    let trace = Scatter::new(time.clone(), xvec)
        .name("Biomass Concentration")
        .mode(plotly::common::Mode::Lines)
        .line(plotly::common::Line::new().color("red").width(2.));

    let trace3 = Scatter::new(time.clone(), s.slice(s![..]).to_vec())
        .name("Glucose Concentration")
        .mode(plotly::common::Mode::Lines)
        .line(plotly::common::Line::new().color("blue").width(2.));

    let layout = Layout::new()
        .title("Concentration Over Time")
        .x_axis(
            plotly::layout::Axis::new()
                .title("Time (h)"),
        )
        .y_axis(plotly::layout::Axis::new().title("Concentration (g/L)"))
        .width(800)
        .height(800)
        .margin(plotly::layout::Margin::new());

    plot.add_trace(trace);
    plot.add_trace(trace3);

    plot.set_layout(layout);

    plot.write_image("./examples/out_rust.svg", ImageFormat::SVG, 800, 600, 1.0);
    plot.write_html("./examples/out_rust.html");

    println!("{}", plot.to_inline_html(Some("div_plot_x")));

    println!(
        "{}",
        obj.tallies().expect("REASON").to_csv().expect("REASE")
    );
}
