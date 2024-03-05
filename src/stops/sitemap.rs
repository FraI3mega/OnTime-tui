use color_eyre::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Sitemap {
    pub main_url: String,
    pub buses: String,
    pub stops: String,
    pub lines: String,
    pub alerts: String,
    pub vehicles: String,
    pub real_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SitemapRaw {
    alerts: String,
    all_messages: String,
    atomic_schedule: String,
    attachment: String,
    base: String,
    buses: String,
    calendar: String,
    course_details: String,
    direction: String,
    image: String,
    lines: String,
    main_variants_by_line_name: String,
    map_providers: String,
    mapped_variant_names: String,
    message_categories: String,
    messageContent: String,
    messages: String,
    real_course: String,
    regulations: String,
    real_time: String,
    running_vehicles: String,
    schedule: String,
    stops: String,
    theoretical_course_details: String,
    traffic: String,
    traffic_thresholds: String,
    transfers: String,
    trip: String,
    variant_letters_by_id: String,
    vehicles: String,
}

pub fn get_sitemap(main_url: String) -> Result<Sitemap> {
    let data = ureq::get(format!("{}getCities.json", main_url).as_str())
        .call()?
        .into_json::<serde_json::Value>()?;
    dbg!(&data["cities"][0]["url"]);
    let sitemap_raw: SitemapRaw = serde_json::from_value(data["cities"][0]["url"].clone())?;
    Ok(Sitemap {
        main_url,
        buses: sitemap_raw.buses,
        stops: sitemap_raw.stops,
        lines: sitemap_raw.lines,
        alerts: sitemap_raw.alerts,
        vehicles: sitemap_raw.vehicles,
        real_time: sitemap_raw.real_time,
    })
}
