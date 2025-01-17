use clap::Parser;
use serde::Serialize;
use std::error::Error;

use cli_args::{Cli, Command};
use geo_info::GeoInfo;
use salah::prelude::*;

mod cli_args;
mod geo_info;

#[derive(Debug, Serialize)]
struct PrayerWithTime {
    name: String,
    hours: u32,
    minutes: u32,
}
#[derive(Debug, Serialize)]
struct PrayerWithTimeAbs {
    name: String,
    hour: u32,
    minute: u32,
    x: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let now = chrono::Local::now();
    let geo_data = GeoInfo::new().await?;
    let location = Coordinates::new(geo_data.lat, geo_data.lon);
    let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    let params = Configuration::with(Method::MoonsightingCommittee, Madhab::Hanafi);
    let prayer = PrayerSchedule::new()
        .on(date)
        .for_location(location)
        .with_configuration(params)
        .calculate()?;

    match cli.command {
        Command::Next { json } => {
            let next_prayer = prayer.next().name();
            let (hours, minutes) = prayer.time_remaining();
            if !json {
                println!("{hours} hours and {minutes} minutes until {next_prayer}",);
                return Ok(());
            }

            let prayer = PrayerWithTime {
                name: next_prayer.to_string(),
                hours,
                minutes,
            };
            println!("{}", serde_json::to_string(&prayer).unwrap());
        }
        Command::Today { json } => {
            let prayers = [
                Prayer::Fajr,
                Prayer::Dhuhr,
                Prayer::Asr,
                Prayer::Maghrib,
                Prayer::Isha,
                Prayer::Qiyam,
                Prayer::FajrTomorrow,
            ]
            .iter()
            .map(|i| {
                let name = i.name();
                let tz = Local::now().timezone();
                let time = prayer.time(*i).with_timezone(&tz);
                let (pm, hour) = time.hour12();
                let pm = match pm {
                    true => "PM",
                    false => "AM",
                };
                let minute = time.minute();
                PrayerWithTimeAbs {
                    name,
                    hour,
                    minute,
                    x: pm.to_owned(),
                }
            });
            if !json {
                for i in prayers {
                    let name = i.name;
                    let hour = i.hour;
                    let minute = i.minute;
                    let x = i.x;
                    println!("{name} at {hour}{x}:{minute}")
                }
                return Ok(());
            }
            println!(
                "{}",
                serde_json::to_string(&prayers.collect::<Vec<_>>()).unwrap()
            );
            return Ok(());
        }
    }
    Ok(())
}
