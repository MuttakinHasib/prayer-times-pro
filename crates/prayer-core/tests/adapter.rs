//! Verifies each adapter emits the prescribed parameters, and that the registry
//! resolves ids, the Hanafi modifier, and country mappings.

use prayer_core::{
    CalculationMethodAdapter, CalculationParameters, Coordinates, DiyanetAdapter, EgyptianAdapter,
    HanafiAsrModifier, IsnaAdapter, JakimAdapter, KarachiAdapter, KemenagAdapter, ManualAdapter,
    MethodRegistry, MwlAdapter, Prayer, UmmAlQuraAdapter,
};

fn anywhere() -> Coordinates {
    Coordinates::new(41.0, 29.0)
}

#[test]
fn diyanet_parameters() {
    let p = DiyanetAdapter.resolve(anywhere());
    assert_eq!(p.fajr_angle, 18.0);
    assert_eq!(p.isha_angle, Some(17.0));
    assert_eq!(p.sunrise_angle, -1.9);
    assert_eq!(p.asr_shadow_factor, 1.0);
    assert_eq!(p.dhuhr_offset_minutes, 5);
    assert_eq!(p.asr_offset_minutes, 4);
}

#[test]
fn angle_based_methods() {
    assert_eq!(MwlAdapter.resolve(anywhere()).fajr_angle, 18.0);
    assert_eq!(MwlAdapter.resolve(anywhere()).isha_angle, Some(17.0));
    assert_eq!(IsnaAdapter.resolve(anywhere()).fajr_angle, 15.0);
    assert_eq!(EgyptianAdapter.resolve(anywhere()).fajr_angle, 19.5);
    assert_eq!(EgyptianAdapter.resolve(anywhere()).isha_angle, Some(17.5));
    assert_eq!(KarachiAdapter.resolve(anywhere()).isha_angle, Some(18.0));
}

#[test]
fn jakim_calibrated_parameters() {
    let p = JakimAdapter.resolve(anywhere());
    assert_eq!(p.fajr_angle, 17.5);
    assert_eq!(p.isha_angle, Some(18.0));
    assert_eq!(p.asr_shadow_factor, 1.0);
    assert_eq!(p.dhuhr_offset_minutes, 3);
    assert_eq!(p.asr_offset_minutes, 2);
    assert_eq!(p.manual_offsets.get(&Prayer::Maghrib), Some(&2));
    assert_eq!(p.manual_offsets.get(&Prayer::Isha), Some(&2));
}

#[test]
fn kemenag_calibrated_parameters() {
    let p = KemenagAdapter.resolve(anywhere());
    assert_eq!(p.fajr_angle, 20.0);
    assert_eq!(p.isha_angle, Some(18.0));
    assert_eq!(p.asr_shadow_factor, 1.0);
    assert_eq!(p.dhuhr_offset_minutes, 3);
    assert_eq!(p.asr_offset_minutes, 2);
    assert_eq!(p.manual_offsets.get(&Prayer::Fajr), Some(&2));
    assert_eq!(p.manual_offsets.get(&Prayer::Maghrib), Some(&3));
    assert_eq!(p.manual_offsets.get(&Prayer::Isha), Some(&2));
}

#[test]
fn umm_al_qura_uses_fixed_isha() {
    let p = UmmAlQuraAdapter.resolve(anywhere());
    assert_eq!(p.fajr_angle, 18.5);
    assert_eq!(p.isha_angle, None);
    assert_eq!(p.isha_fixed_minutes, Some(90));
}

#[test]
fn hanafi_modifier_only_changes_asr() {
    let bp = MwlAdapter.resolve(anywhere());
    let modified = HanafiAsrModifier::new(Box::new(MwlAdapter));
    let mp = modified.resolve(anywhere());

    assert_eq!(mp.asr_shadow_factor, 2.0);
    assert_eq!(modified.id(), "mwl.hanafi");
    assert_eq!(mp.fajr_angle, bp.fajr_angle);
    assert_eq!(mp.isha_angle, bp.isha_angle);
    assert_eq!(mp.sunrise_angle, bp.sunrise_angle);
}

#[test]
fn manual_adapter_passes_parameters_through() {
    let mut custom = CalculationParameters::new(12.0);
    custom.isha_angle = Some(12.0);
    custom.asr_shadow_factor = 2.0;
    custom.dhuhr_offset_minutes = 3;
    let adapter = ManualAdapter::new(custom.clone());
    assert_eq!(adapter.resolve(anywhere()), custom);
}

#[test]
fn registry_resolves_built_in_by_id() {
    let adapter = MethodRegistry::resolve("diyanet", false, None).unwrap();
    assert_eq!(adapter.id(), "diyanet");
}

#[test]
fn registry_applies_hanafi() {
    let adapter = MethodRegistry::resolve("isna", true, None).unwrap();
    assert_eq!(adapter.id(), "isna.hanafi");
    assert_eq!(adapter.resolve(anywhere()).asr_shadow_factor, 2.0);
}

#[test]
fn registry_resolves_manual_with_parameters() {
    let mut custom = CalculationParameters::new(16.0);
    custom.isha_angle = Some(16.0);
    let adapter = MethodRegistry::resolve("manual", false, Some(custom)).unwrap();
    assert_eq!(adapter.id(), "manual");
    assert_eq!(adapter.resolve(anywhere()).fajr_angle, 16.0);
}

#[test]
fn registry_returns_none_for_unknown_id() {
    assert!(MethodRegistry::resolve("does-not-exist", false, None).is_none());
}

#[test]
fn country_method_mapping() {
    assert_eq!(MethodRegistry::method_id_for_country(Some("TR")), "diyanet");
    assert_eq!(MethodRegistry::method_id_for_country(Some("us")), "isna"); // case-insensitive
    assert_eq!(MethodRegistry::method_id_for_country(Some("SA")), "ummalqura");
    assert_eq!(MethodRegistry::method_id_for_country(Some("PK")), "karachi");
    assert_eq!(MethodRegistry::method_id_for_country(Some("MY")), "jakim");
    assert_eq!(MethodRegistry::method_id_for_country(Some("ID")), "kemenag");
    assert_eq!(MethodRegistry::method_id_for_country(Some("ZZ")), "mwl");
    assert_eq!(MethodRegistry::method_id_for_country(None), "mwl");
}

#[test]
fn built_in_excludes_manual() {
    assert!(!MethodRegistry::built_in().iter().any(|a| a.id() == "manual"));
    assert_eq!(MethodRegistry::built_in().len(), 9);
}
