use crate::{find_target, Plot};
use anyhow::{Context, Result};
use std::{env, process};

impl Plot {
    pub fn generate_plot(&mut self) -> Result<(), anyhow::Error> {
        eprintln!("Generating plot");

        self.target =
            find_target(&self.target).context("⚠️  couldn't find the target for plotting")?;

        // The cargo executable
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

        let fuzzer_data_dir = format!(
            "./{}/{}/afl/{}/",
            &self.output.display(),
            &self.target,
            &self.input
        );

        let plot_dir = self
            .plot
            .display()
            .to_string()
            .replace("{output}", &self.output.display().to_string())
            .replace("{target_name}", &self.target);
        println!("{plot_dir}");
        println!("{}", self.target);

        // We run the afl-plot command
        process::Command::new(cargo)
            .args(["afl", "plot", &fuzzer_data_dir, &plot_dir])
            .spawn()
            .context("⚠️  couldn't spawn afl plot")?
            .wait()
            .context("⚠️  couldn't wait for the afl plot")?;

        Ok(())
    }
}
