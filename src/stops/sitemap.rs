use color_eyre::Result;

pub struct Sitemap {
    main_url: String,
    buses: String,
    stops: String,
    lines: String,
    alerts: String,
    vehicles: String,
    real_time: String,
}

pub fn get_sitemap(main_url: String) -> Result<Sitemap> {
    let data = ureq::get(format!("{}getCities.json", main_url).as_str())
        .call()?
        .into_json::<serde_json::Value>()?;
    Ok(Sitemap {
        main_url,
        buses: data["buses"].to_string(),
        stops: data["stops"].to_string(),
        lines: data["lines"].to_string(),
        alerts: data["alerts"].to_string(),
        vehicles: data["vehicles"].to_string(),
        real_time: data["realTime"].to_string(),
    })
}
