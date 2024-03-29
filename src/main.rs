mod cli;
mod stops;

use std::collections::HashMap;

use clap::Parser;
use color_eyre::Result;
use comfy_table::Table;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    cli::Cli,
    stops::{get_stop_data, get_stops, select_stop, sitemap::get_sitemap},
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    main_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main_url: "https://dip.mzkopole.pl/".to_string(),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let config: Config = confy::load("ontime-tui", Some("config"))?;
    let sitemap = get_sitemap(match cli.main_url {
        Some(n) => n,
        None => config.main_url,
    })?;

    let stops: HashMap<String, u32> = get_stops(sitemap.clone())?;
    let stop_number = match cli.stop_number {
        Some(n) => n,
        None => select_stop(stops)?,
    };

    let stop_data = get_stop_data(sitemap, stop_number)?;
    let mut table = Table::new();
    table.set_header(vec!["Line number", "Line name", "Arrives in"]);

    for bus in stop_data.departures {
        let departs_in = OffsetDateTime::from_unix_timestamp(bus.real_departure / 1000)?
            - OffsetDateTime::now_utc();

        table.add_row(vec![
            bus.line_name,
            bus.direction_name,
            format!("{}m", departs_in.whole_minutes().to_string()),
        ]);
    }

    println!("\n{}", table);
    Ok(())
}
