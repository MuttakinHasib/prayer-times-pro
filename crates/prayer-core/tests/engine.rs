//! Validates the astronomical core against independently verifiable data and the
//! method-specific offset handling. Ported from PrayerKit `EngineTests`.

mod common;

use common::{date, minutes_of_day, tz};
use prayer_core::{
    calculate, CalculationMethodAdapter, Coordinates, DiyanetAdapter, HanafiAsrModifier,
    HighLatitudeRule, KarachiAdapter, MwlAdapter, Prayer, UmmAlQuraAdapter,
};

/// Raleigh, NC — 2015-07-12, MWL, Shafi Asr, America/New_York. Reference values
/// from NOAA/timeanddate (sunrise/noon/sunset/twilight) plus a hand-computed Asr.
#[test]
fn mwl_reference_raleigh() {
    let zone = tz("America/New_York");
    let coords = Coordinates::new(35.7750, -78.6336);
    let t = calculate(date(2015, 7, 12), coords, &MwlAdapter.resolve(coords), zone);

    let check = |prayer: Prayer, expected: &str, tol: i64, name: &str| {
        let (h, m) = expected.split_once(':').unwrap();
        let exp = h.parse::<i64>().unwrap() * 60 + m.parse::<i64>().unwrap();
        let actual = minutes_of_day(t.get(prayer).expect(name));
        assert!((actual - exp).abs() <= tol, "{name}: got {actual} want {exp}");
    };
    check(Prayer::Fajr, "04:21", 3, "Fajr");
    check(Prayer::Sunrise, "06:08", 2, "Sunrise");
    check(Prayer::Dhuhr, "13:20", 2, "Dhuhr");
    check(Prayer::Asr, "17:09", 2, "Asr");
    check(Prayer::Maghrib, "20:32", 2, "Maghrib");
    check(Prayer::Isha, "22:10", 3, "Isha");
}

#[test]
fn times_are_chronological() {
    let zone = tz("Europe/Istanbul");
    let coords = Coordinates::new(41.0082, 28.9784);
    let t = calculate(date(2024, 3, 21), coords, &DiyanetAdapter.resolve(coords), zone);
    let order = [
        Prayer::Fajr,
        Prayer::Sunrise,
        Prayer::Dhuhr,
        Prayer::Asr,
        Prayer::Maghrib,
        Prayer::Isha,
    ];
    let dates: Vec<_> = order.iter().filter_map(|&p| t.get(p)).collect();
    assert_eq!(dates.len(), 6, "all six times present");
    for w in dates.windows(2) {
        assert!(w[0] < w[1], "times strictly increasing");
    }
}

#[test]
fn hanafi_asr_is_later_than_standard() {
    let zone = tz("Asia/Karachi");
    let coords = Coordinates::new(24.8607, 67.0011);
    let standard = calculate(date(2024, 1, 15), coords, &KarachiAdapter.resolve(coords), zone);
    let hanafi = calculate(
        date(2024, 1, 15),
        coords,
        &HanafiAsrModifier::new(Box::new(KarachiAdapter)).resolve(coords),
        zone,
    );
    assert!(hanafi.get(Prayer::Asr).unwrap() > standard.get(Prayer::Asr).unwrap());
}

#[test]
fn umm_al_qura_fixed_isha_is_maghrib_plus_90() {
    let zone = tz("Asia/Riyadh");
    let coords = Coordinates::new(21.4225, 39.8262); // Makkah
    let t = calculate(date(2024, 5, 10), coords, &UmmAlQuraAdapter.resolve(coords), zone);
    let gap = (t.get(Prayer::Isha).unwrap() - t.get(Prayer::Maghrib).unwrap()).num_seconds();
    assert_eq!(gap, 90 * 60, "Isha is Maghrib + 90 min");
}

#[test]
fn diyanet_ihtiyat_offsets() {
    let zone = tz("Europe/Istanbul");
    let coords = Coordinates::new(41.0082, 28.9784);
    let d = date(2024, 9, 1);

    let mut no_offset = DiyanetAdapter.resolve(coords);
    no_offset.dhuhr_offset_minutes = 0;
    no_offset.asr_offset_minutes = 0;

    let with_offset = calculate(d, coords, &DiyanetAdapter.resolve(coords), zone);
    let baseline = calculate(d, coords, &no_offset, zone);

    let ddhuhr = (with_offset.get(Prayer::Dhuhr).unwrap() - baseline.get(Prayer::Dhuhr).unwrap()).num_seconds();
    let dasr = (with_offset.get(Prayer::Asr).unwrap() - baseline.get(Prayer::Asr).unwrap()).num_seconds();
    assert_eq!(ddhuhr, 5 * 60);
    assert_eq!(dasr, 4 * 60);
}

#[test]
fn manual_offsets_applied_last() {
    let zone = tz("Europe/London");
    let coords = Coordinates::new(51.5074, -0.1278);
    let d = date(2024, 4, 1);

    let params = MwlAdapter.resolve(coords);
    let baseline = calculate(d, coords, &params, zone);
    let mut tuned_params = params.clone();
    tuned_params.manual_offsets.insert(Prayer::Fajr, -3);
    tuned_params.manual_offsets.insert(Prayer::Isha, 7);
    let tuned = calculate(d, coords, &tuned_params, zone);

    assert_eq!(
        (tuned.get(Prayer::Fajr).unwrap() - baseline.get(Prayer::Fajr).unwrap()).num_seconds(),
        -3 * 60
    );
    assert_eq!(
        (tuned.get(Prayer::Isha).unwrap() - baseline.get(Prayer::Isha).unwrap()).num_seconds(),
        7 * 60
    );
    assert_eq!(
        tuned.get(Prayer::Dhuhr).unwrap(),
        baseline.get(Prayer::Dhuhr).unwrap(),
        "untouched prayers unchanged"
    );
}

#[test]
fn high_latitude_rule_fills_missing_times() {
    let zone = tz("Europe/Oslo");
    let coords = Coordinates::new(59.9139, 10.7522); // Oslo
    let d = date(2024, 6, 21); // solstice

    let mut none = MwlAdapter.resolve(coords);
    none.high_latitude_rule = HighLatitudeRule::None;
    let raw = calculate(d, coords, &none, zone);

    let mut angle = MwlAdapter.resolve(coords);
    angle.high_latitude_rule = HighLatitudeRule::AngleBased;
    let adjusted = calculate(d, coords, &angle, zone);

    assert!(raw.get(Prayer::Fajr).is_none(), "no true astronomical Fajr at Oslo on the solstice");
    assert!(adjusted.get(Prayer::Fajr).is_some(), "angle-based supplies a Fajr");
    assert!(adjusted.get(Prayer::Isha).is_some(), "angle-based supplies an Isha");
    assert!(adjusted.get(Prayer::Fajr).unwrap() < adjusted.get(Prayer::Sunrise).unwrap());
}

#[test]
fn krakow_june_angle_based_fajr_and_isha() {
    let zone = tz("Europe/Warsaw");
    let coords = Coordinates::new(50.0532, 19.9443);
    let d = date(2026, 6, 5);

    let params = MwlAdapter.resolve(coords);
    assert_eq!(params.high_latitude_rule, HighLatitudeRule::AngleBased);
    let t = calculate(d, coords, &params, zone);

    let fajr = t.get(Prayer::Fajr).expect("angle-based supplies a Fajr");
    let isha = t.get(Prayer::Isha).expect("angle-based supplies an Isha");
    assert!(fajr < t.get(Prayer::Sunrise).unwrap(), "Fajr before sunrise");
    assert!(t.get(Prayer::Maghrib).unwrap() < isha, "Isha after Maghrib");

    // Reference (aladhan MWL): Fajr ~2:13 AM, Isha ~10:57 PM. Allow ±15 min.
    assert!((minutes_of_day(fajr) - (2 * 60 + 13)).abs() <= 15, "Fajr near 2:13 AM");
    assert!((minutes_of_day(isha) - (22 * 60 + 57)).abs() <= 15, "Isha near 10:57 PM");
}
