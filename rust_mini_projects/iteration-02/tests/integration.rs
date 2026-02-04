use std::collections::HashSet;

use it02_data_processing::tasks::{
    extreme_accidents, missing_units, night_times, unbuckled_trend, unfavorable_accident,
};

#[test]
fn test_night_times_ok() {
    use chrono::NaiveTime;

    let (expected_start, expected_end) = (
        NaiveTime::from_hms_opt(21, 00, 00).unwrap(),
        NaiveTime::from_hms_opt(5, 59, 00).unwrap(),
    );
    let (actual_start, actual_end) = night_times();

    assert_eq!(expected_start, actual_start);
    assert_eq!(expected_end, actual_end);
}

#[test]
fn test_unfavorable_accident_ok() {
    let expected = Some(String::from("11. 7. 2020 2:45"));
    let actual = unfavorable_accident();

    assert_eq!(expected, actual)
}

#[test]
fn test_unbuckled_trend_ok() {
    let expected = "^^^vvv^vvvvv";
    let actual = unbuckled_trend();

    assert_eq!(expected, actual);
}

#[test]
fn test_extreme_accidents_ok() {
    let (expected_string, expected_bool) = ("5360000 Kč : 11 vehicles : 43 people", false);
    let (actual_string, actual_bool) = extreme_accidents();

    assert_eq!(expected_string, actual_string);
    assert_eq!(expected_bool, actual_bool);
}

#[test]
fn test_missing_units_ok() {
    let expected = HashSet::from_iter(
        [
            "Palackého vrch",
            "Zouvalka",
            "Holedná",
            "Padělky",
            "Zaječí hora",
            "Pohádka máje",
            "Mladá hora",
        ]
        .into_iter()
        .map(|result| result.to_string()),
    );

    let actual = missing_units();

    assert_eq!(expected, actual);
}
