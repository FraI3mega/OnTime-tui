use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct StopData {
    stopPointSymbol: String,
    stopPointId: u8,
    stopPointName: String,
    responseDate: usize,
    departures: Vec<BusData>,
}

#[derive(Deserialize, Debug)]
struct BusData {
    courseId: usize,
    scheduledDepartureSec: u32,
    scheduledDeparture: usize,
    realDeparture: usize,
    vehicleId: String,
    variantId: u32,
    orderInCourse: u8,
    passed: bool,
    lack: bool,
    onStopPoint: bool,
    lineName: String,
    directionName: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    //TODO: Add stop selection
    let stop = 263;
    println!("{:#?}", get_stop_data(stop)?);
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
