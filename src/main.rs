use std::collections::HashMap;
use std::io::Cursor;

use color_eyre::eyre::Result;
use comfy_table::Table;
use serde::Deserialize;
use skim::prelude::*;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StopData {
    _stop_point_symbol: String,
    _stop_point_id: u16,
    stop_point_name: String,
    _response_date: usize,
    departures: Vec<BusData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BusData {
    _course_id: usize,
    _scheduled_departure_sec: u32,
    _scheduled_departure: usize,
    real_departure: i64, // NOTE: this in in miliseconds
    _vehicle_id: String,
    _variant_id: u32,
    _order_in_course: u16,
    _passed: bool,
    _lack: bool,
    _on_stop_point: bool,
    line_name: String,
    direction_name: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let stop_str = include_str!("stops.json");
    let stops: HashMap<String, u16> = serde_json::from_str(stop_str)?;
    // TODO: Add stop selection using stops and skim
    let stop_number = select_stop(stops)?;

    let stop_data = get_stop_data(stop_number)?;
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

    println!(
        "Stop id: {}, stop name: {}",
        stop_number, stop_data.stop_point_name
    );
    println!("{}", table);
    Ok(())
}

fn get_stop_data(stop_number: u16) -> Result<StopData> {
    let data: StopData = ureq::get(
        format!(
            "https://dip.mzkopole.pl/getRealtime.json?stopPointSymbol={}",
            stop_number
        )
        .as_str(),
    )
    .call()?
    .into_json()?;

    Ok(data)
}

fn select_stop(stops: HashMap<String, u16>) -> Result<u16> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = stops
        .keys()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    Ok(*stops.get(&selected_items[0].output().to_string()).unwrap())
}
