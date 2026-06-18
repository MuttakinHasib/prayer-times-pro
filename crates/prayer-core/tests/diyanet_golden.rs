//! Appendix A hard gate: the Diyanet adapter must reproduce the official Diyanet
//! monthly tables for Istanbul/Başakşehir, Ankara, and Istanbul/Arnavutköy to
//! within ±1 minute for all six times across a full month.
//!
//! Data-driven: reads `tests/fixtures/diyanet_golden_tables.json` and checks
//! every row (93 days across 3 cities).

mod common;

use common::{date, hm, minutes_of_day, tz};
use prayer_core::{calculate, CalculationMethodAdapter, Coordinates, DiyanetAdapter, Prayer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GoldenFile {
    cases: Vec<GoldenCase>,
}
#[derive(Deserialize)]
struct GoldenCase {
    city: String,
    latitude: f64,
    longitude: f64,
    elevation: Option<f64>,
    #[serde(rename = "timeZone")]
    time_zone: String,
    days: Vec<GoldenDay>,
}
#[derive(Deserialize)]
struct GoldenDay {
    date: String,
    fajr: String,
    sunrise: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

fn parse_date(s: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = s.split('-').collect();
    (
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].parse().unwrap(),
    )
}

#[test]
fn diyanet_matches_official_tables_within_one_minute() {
    let raw = include_str!("fixtures/diyanet_golden_tables.json");
    let cases = serde_json::from_str::<GoldenFile>(raw).expect("golden JSON parses").cases;
    assert!(!cases.is_empty(), "golden tables present");

    let adapter = DiyanetAdapter;
    for golden in &cases {
        let zone = tz(&golden.time_zone);
        let coords = Coordinates::with_elevation(
            golden.latitude,
            golden.longitude,
            golden.elevation.unwrap_or(0.0),
        );
        let params = adapter.resolve(coords);
        for day in &golden.days {
            let (y, m, d) = parse_date(&day.date);
            let times = calculate(date(y, m, d), coords, &params, zone);
            let label = format!("{} {}", golden.city, day.date);
            for (prayer, expected, name) in [
                (Prayer::Fajr, &day.fajr, "Fajr"),
                (Prayer::Sunrise, &day.sunrise, "Sunrise"),
                (Prayer::Dhuhr, &day.dhuhr, "Dhuhr"),
                (Prayer::Asr, &day.asr, "Asr"),
                (Prayer::Maghrib, &day.maghrib, "Maghrib"),
                (Prayer::Isha, &day.isha, "Isha"),
            ] {
                let actual = times
                    .get(prayer)
                    .map(minutes_of_day)
                    .unwrap_or_else(|| panic!("{label} {name}: missing"));
                let exp = hm(expected);
                assert!(
                    (actual - exp).abs() <= 1,
                    "{label} {name}: ours {actual} vs Diyanet {exp} (Δ{} min)",
                    actual - exp
                );
            }
        }
    }
}
