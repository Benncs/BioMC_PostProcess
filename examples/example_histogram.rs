use bcore::api::PostProcessPopulation;
use bcore::{PostProcess,PostProcessReader,PostProcessReaderInfo};
use plotly::ImageFormat;
use plotly::{Bar, Layout, Plot};

fn main() {
    if let Ok(obj) = PostProcess::new(
        "example0d",
        Some(
            "./examples/"
                .to_string(),
        ),
    ) {
        // Generate histogram data for "nu_eff"
        let (nu_eff_bins, nu_eff_counts) = obj
            .get_histogram(100, obj.n_export() - 1, "nu_eff_2")
            .unwrap();
        let nu_eff_bin_converted: Vec<f64> = nu_eff_bins.iter().map(|x| *x ).collect();

        // Create bar trace for "nu_eff"
        let nu_eff_bar = Bar::new(nu_eff_bin_converted.clone(), nu_eff_counts)
            .name("Frequency (nu_eff)")
            .marker(plotly::common::Marker::new().color("steelblue"));

        // Generate histogram data for "nu_meta"
        let (nu_meta_bins, nu_meta_counts) = obj
            .get_histogram(100, obj.n_export() - 1, "nu2")
            .unwrap();
        let nu_meta_bin_converted: Vec<f64> = nu_meta_bins.iter().map(|x| *x ).collect();

        // Create bar trace for "nu_meta"
        let nu_meta_bar = Bar::new(nu_meta_bin_converted.clone(), nu_meta_counts)
            .name("Frequency (nu_meta)")
            .marker(plotly::common::Marker::new().color("black"));

        let mean_nu_meta = obj.get_population_mean("nu_eff_2", obj.n_export()-1).unwrap();
   

    
        // Define layout with enhanced styling
        let layout = Layout::new()
            .title("Final \\nu_{eff}")
            .x_axis(
                plotly::layout::Axis::new()
                    .title("Bin Midpoints (seconds)")
                    .grid_color("lightgray"),
            )
            .y_axis(
                plotly::layout::Axis::new()
                    .title("Counts")
                    .grid_color("lightgray"),
            )
            .bar_mode(plotly::layout::BarMode::Overlay)
            .width(1000)
            .height(1000)
            .show_legend(true); // Ensures the legend is displayed

        // Combine traces and layout into a plot
        let mut plot = Plot::new();
        plot.add_trace(nu_eff_bar);
        plot.add_trace(nu_meta_bar);
  

        plot.set_layout(layout);

        // Display or export the plot

        plot.write_html("./examples/out_hist.html");
        plot.write_image("./examples/out_hist.svg", ImageFormat::SVG, 1000, 1000, 1.0);

        plot.show();
    } else {
        println!("Simulation not found");
    }
}
