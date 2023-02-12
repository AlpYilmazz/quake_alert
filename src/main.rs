use alert::{matches_alert_rule, send_alert_mail, AlertContent};
use cache::LastQuake;
use config::ProgramConfig;
use fetch::{fetch_quake_data, Source};
use process::process_quake_data;

pub mod alert;
pub mod cache;
pub mod config;
pub mod fetch;
pub mod process;
pub mod util;

fn execute_single(last_quake: &mut LastQuake, source: Source) {
    let program_config = ProgramConfig::get();

    let Ok(quake_data) = fetch_quake_data(source) else {
        return;
    };
    let quake_data = process_quake_data(quake_data);

    let new_last_quake = quake_data.get(0).cloned();

    let quake_data = match &last_quake.0 {
        Some(last_quake) => quake_data
            .into_iter()
            .take_while(|q| !q.same_instance(last_quake))
            .collect::<Vec<_>>(),
        None => quake_data,
    };

    *last_quake = LastQuake(new_last_quake);

    let alert_quakes = quake_data
        .iter()
        .filter(|qd| matches_alert_rule(&program_config.rule, qd))
        .cloned()
        .collect::<Vec<_>>();

    if alert_quakes.is_empty() {
        #[cfg(debug_assertions)]
        println!("No quakes to alert");
        return;
    }
    #[cfg(debug_assertions)]
    println!("There are some quakes to alert");

    let alerts = alert_quakes
        .iter()
        .map(|quake_data| AlertContent {
            date: quake_data.date.clone(),
            time: quake_data.time.clone(),
            location: quake_data.location.clone(),
            magnitude: quake_data.magnitude.get_main(),
        })
        .collect::<Vec<_>>();

    send_alert_mail(&program_config.account, &alerts);
}

const CONFIG_PATH: &'static str = "config.ron";
const CACHE_PATH: &'static str = "data/last_quake";
const _LOCAL_DATA_PATH: &'static str = "data/quakes";
const _REMOTE_DATA_URL: &'static str = "http://www.koeri.boun.edu.tr/scripts/sondepremler.asp";

fn main() {
    ProgramConfig::initialize(CONFIG_PATH);

    let mut last_quake = LastQuake::load(CACHE_PATH);

    loop {
        execute_single(
            &mut last_quake,
            // Source::Local(_LOCAL_DATA_PATH),
            Source::Remote(_REMOTE_DATA_URL),
        );
        let _ = last_quake.save(CACHE_PATH);
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
