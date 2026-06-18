//! Hard gate for the Kemenag adapter: it must reproduce Kemenag's official
//! published prayer times for KOTA JAKARTA to within ±1 minute for Subuh,
//! Dzuhur, Ashar, Maghrib, and Isya. Reference rows span solstices/equinoxes.
//!
//! Terbit (sunrise) is not gated: Kemenag shaves ihtiyati off it.

mod common;

use common::{date, hm, minutes_of_day, tz};
use prayer_core::{calculate, CalculationMethodAdapter, Coordinates, KemenagAdapter, Prayer};

struct Row {
    y: i32,
    m: u32,
    d: u32,
    subuh: &'static str,
    dzuhur: &'static str,
    ashar: &'static str,
    maghrib: &'static str,
    isya: &'static str,
}

#[test]
fn kemenag_matches_official_tables_within_one_minute() {
    // Kemenag reference point for KOTA JAKARTA (back-solved from the tables).
    let jakarta = Coordinates::new(-6.21, 106.72);
    let zone = tz("Asia/Jakarta");

    let golden = [
        Row { y: 2026, m: 3, d: 21, subuh: "04:42", dzuhur: "12:04", ashar: "15:14", maghrib: "18:07", isya: "19:15" },
        Row { y: 2026, m: 6, d: 6, subuh: "04:37", dzuhur: "11:55", ashar: "15:16", maghrib: "17:48", isya: "19:02" },
        Row { y: 2026, m: 9, d: 23, subuh: "04:27", dzuhur: "11:49", ashar: "14:58", maghrib: "17:52", isya: "19:00" },
        Row { y: 2026, m: 12, d: 21, subuh: "04:13", dzuhur: "11:54", ashar: "15:20", maghrib: "18:08", isya: "19:24" },
    ];

    let params = KemenagAdapter.resolve(jakarta);
    for g in &golden {
        let t = calculate(date(g.y, g.m, g.d), jakarta, &params, zone);
        let label = format!("{}-{}-{}", g.y, g.m, g.d);
        for (prayer, expected, name) in [
            (Prayer::Fajr, g.subuh, "Subuh"),
            (Prayer::Dhuhr, g.dzuhur, "Dzuhur"),
            (Prayer::Asr, g.ashar, "Ashar"),
            (Prayer::Maghrib, g.maghrib, "Maghrib"),
            (Prayer::Isha, g.isya, "Isya"),
        ] {
            let actual = t
                .get(prayer)
                .map(minutes_of_day)
                .unwrap_or_else(|| panic!("{label}: missing {name}"));
            let exp = hm(expected);
            assert!(
                (actual - exp).abs() <= 1,
                "{label} {name}: ours {actual} vs Kemenag {exp} (Δ{} min)",
                actual - exp
            );
        }
    }
}
