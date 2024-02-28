use color_eyre::eyre::Result;
use comfy_table::Table;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StopData {
    _stop_point_symbol: String,
    _stop_point_id: u8,
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
    real_departure: i64,
    _vehicle_id: String,
    _variant_id: u32,
    _order_in_course: u8,
    _passed: bool,
    _lack: bool,
    _on_stop_point: bool,
    line_name: String,
    direction_name: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    //TODO: Add stop selection
    let stop_number = 263;

    let stop_data = get_stop_data(stop_number)?;
    let mut table = Table::new();
    table.set_header(vec!["Line number", "Line name", "Arrives in"]);

    for bus in stop_data.departures {
        let departs_in = OffsetDateTime::from_unix_timestamp(bus.real_departure / 1000)?
            - OffsetDateTime::now_local()?;

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
