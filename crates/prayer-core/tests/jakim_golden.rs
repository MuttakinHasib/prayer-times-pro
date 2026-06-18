//! Hard gate for the JAKIM adapter: it must reproduce JAKIM's official e-Solat
//! output for Kuala Lumpur (zone WLY01) to within ±1 minute for Fajr, Dhuhr,
//! Asr, Maghrib, and Isha. Reference rows span both solstices and equinoxes.
//!
//! Sunrise (syuruk) is intentionally not gated: e-Solat shaves ~1 min off it.

mod common;

use common::{date, hm, minutes_of_day_rounded, tz};
use prayer_core::{calculate, CalculationMethodAdapter, Coordinates, JakimAdapter, Prayer};

struct Row {
    y: i32,
    m: u32,
    d: u32,
    fajr: &'static str,
    dhuhr: &'static str,
    asr: &'static str,
    maghrib: &'static str,
    isha: &'static str,
}

#[test]
fn jakim_matches_esolat_within_one_minute() {
    let kl = Coordinates::new(3.1409, 101.6932);
    let zone = tz("Asia/Kuala_Lumpur");

    // Official JAKIM e-Solat rows (WLY01), spanning the full seasonal range.
    let golden = [
        Row { y: 2026, m: 3, d: 21, fajr: "06:10", dhuhr: "13:23", asr: "16:29", maghrib: "19:26", isha: "20:34" },
        Row { y: 2026, m: 6, d: 6, fajr: "05:50", dhuhr: "13:15", asr: "16:40", maghrib: "19:23", isha: "20:39" },
        Row { y: 2026, m: 9, d: 23, fajr: "05:55", dhuhr: "13:09", asr: "16:14", maghrib: "19:11", isha: "20:20" },
        Row { y: 2026, m: 12, d: 21, fajr: "06:01", dhuhr: "13:14", asr: "16:37", maghrib: "19:11", isha: "20:26" },
    ];

    let params = JakimAdapter.resolve(kl);
    for g in &golden {
        let t = calculate(date(g.y, g.m, g.d), kl, &params, zone);
        let label = format!("{}-{}-{}", g.y, g.m, g.d);
        for (prayer, expected, name) in [
            (Prayer::Fajr, g.fajr, "Fajr"),
            (Prayer::Dhuhr, g.dhuhr, "Dhuhr"),
            (Prayer::Asr, g.asr, "Asr"),
            (Prayer::Maghrib, g.maghrib, "Maghrib"),
            (Prayer::Isha, g.isha, "Isha"),
        ] {
            let actual = t
                .get(prayer)
                .map(minutes_of_day_rounded)
                .unwrap_or_else(|| panic!("{label}: missing {name}"));
            let exp = hm(expected);
            assert!(
                (actual - exp).abs() <= 1,
                "{label} {name}: ours {actual} vs JAKIM {exp} (Δ{} min)",
                actual - exp
            );
        }
    }
}
