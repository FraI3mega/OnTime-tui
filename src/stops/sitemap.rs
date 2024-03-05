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
    _all_messages: String,
    _atomic_schedule: String,
    _attachment: String,
    _base: String,
    buses: String,
    _calendar: String,
    _course_details: String,
    _direction: String,
    _image: String,
    lines: String,
    _main_variants_by_line_name: String,
    _map_providers: String,
    _mapped_variant_names: String,
    _message_categories: String,
    _message_content: String,
    _messages: String,
    _real_course: String,
    _regulations: String,
    real_time: String,
    _running_vehicles: String,
    _schedule: String,
    stops: String,
    _theoretical_course_details: String,
    _traffic: String,
    _traffic_thresholds: String,
    _transfers: String,
    _trip: String,
    _variant_letters_by_id: String,
    vehicles: String,
}

pub fn get_sitemap(main_url: String) -> Result<Sitemap> {
    let data = ureq::get(format!("{}getCities.json", main_url).as_str())
        .call()?
        .into_json::<serde_json::Value>()?;
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
