use eboladrome::car::ParkedCar;
use eboladrome::race::Race;
use eboladrome::track::{Track, TrackSector};
use std::time::{Duration, Instant};

const THRESHOLD: Duration = Duration::from_millis(50);

#[test]
fn correct_standing() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "The Isn't Straight", 90, false),
        TrackSector::new(2, "Your Name Here", 30, true),
        TrackSector::new(3, "The Isn't Straight", 90, false),
        TrackSector::new(4, "Old Lady's House", 40, true),
        TrackSector::new(5, "Bumpy Back Straight", 30, false),
        TrackSector::new(6, "Substation", 4, true),
        TrackSector::new(7, "Field Of Sheep", 4, true),
    ];

    let cars = vec![
        ParkedCar::new(1, "Aston Martin Valkyrie", 1.0),
        ParkedCar::new(2, "McLaren P1", 0.75),
        ParkedCar::new(3, "Ferrari LaFerrari", 0.5),
        ParkedCar::new(4, "Porsche 918 Spyder", 0.25),
    ];

    let t = Track::new_with_safety_car(track, 0.8);
    let r = Race::new(t, 3, cars);
    let standings = r.start_race();
    assert!(standings.iter().map(|cp| cp.id).eq(vec![1, 2, 3, 4]));
}

#[test]
fn single_car_exclusive() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "Tight Curve", 100, true),
        TrackSector::new(2, "Another Tight Curve", 100, true),
        TrackSector::new(3, "Yet Another Tight Curve", 100, true),
        TrackSector::new(4, "Final Tight Curve", 100, true),
    ];

    let cars = vec![ParkedCar::new(1, "Aston Martin Valkyrie", 1.0)];
    let t = Track::new_with_safety_car(track, 0.5);
    let r = Race::new(t, 3, cars);
    let start = Instant::now();
    r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(1600) + THRESHOLD;
    let lower_bound = Duration::from_millis(1200) - THRESHOLD;

    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}

#[test]
fn multiple_cars_parallel() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "Wide Straight", 100, false),
        TrackSector::new(2, "Literally a Landing Strip", 100, false),
        TrackSector::new(3, "Straight", 100, false),
        TrackSector::new(4, "Wide Straight", 100, false),
    ];

    let cars = vec![
        ParkedCar::new(1, "Aston Martin Valkyrie", 1.0),
        ParkedCar::new(2, "McLaren P1", 1.0),
        ParkedCar::new(3, "Ferrari LaFerrari", 1.0),
        ParkedCar::new(4, "Porsche 918 Spyder", 1.0),
    ];

    let t = Track::new_with_safety_car(track, 0.5);
    let r = Race::new(t, 3, cars);
    let start = Instant::now();
    r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(1600) + THRESHOLD;
    let lower_bound = Duration::from_millis(1200) - THRESHOLD;

    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}
