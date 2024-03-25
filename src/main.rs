mod stops;

use std::collections::HashMap;

use color_eyre::Result;
use comfy_table::Table;
use time::OffsetDateTime;

use crate::stops::{get_stop_data, get_stops, select_stop, sitemap::get_sitemap};

fn main() -> Result<()> {
    color_eyre::install()?;
    let sitemap = get_sitemap("https://dip.mzkopole.pl/".to_string())?;
    dbg!(sitemap.clone());

    let stops: HashMap<String, u16> = get_stops(sitemap.clone())?;
    //TODO: implement ui
    Ok(())
}
