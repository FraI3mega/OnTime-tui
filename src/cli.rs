use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Stop number
    #[arg(short, long,value_parser = clap::value_parser!(u32).range(1..))]
    pub stop_number: Option<u32>,

    /// Main url
    #[arg(long, default_value_t = String::from( "https://dip.mzkopole.pl/" ))]
    pub main_url: String,
}
