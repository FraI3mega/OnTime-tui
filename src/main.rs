mod stops;

use std::collections::HashMap;

use color_eyre::Result;
use comfy_table::Table;
use time::OffsetDateTime;

use crate::stops::{get_stop_data, select_stop};

fn main() -> Result<()> {
    color_eyre::install()?;

    let stop_str = include_str!("stops.json");
    let stops: HashMap<String, u16> = serde_json::from_str(stop_str)?;
    let stop_number = select_stop(stops)?;
    let main_url = "https://dip.mzkopole.pl/".to_string();

    let stop_data = get_stop_data(main_url, stop_number)?;
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
