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
        ParkedCar::new(2, "McLaren P1", 0.90),
        ParkedCar::new(3, "Ferrari LaFerrari", 0.80),
        ParkedCar::new(4, "Porsche 918 Spyder", 0.70),
    ];

    let t = Track::new(track);
    let r = Race::new(t, 3, cars);
    let standings = r.start_race();
    assert!(standings.iter().map(|cp| cp.id).eq(vec![1, 2, 3, 4]));
}

#[test]
fn parallel_cars() {
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

    let t = Track::new(track);
    let r = Race::new(t, 3, cars);
    let start = Instant::now();
    r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(1200) + 3 * THRESHOLD;
    let lower_bound = Duration::from_millis(1200) - 3 * THRESHOLD;
    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}

#[test]
fn serial_cars() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "Tight Curve", 100, true),
        TrackSector::new(2, "Another Tight Curve", 100, true),
        TrackSector::new(3, "Yet Another Tight Curve", 100, true),
        TrackSector::new(4, "Final Tight Curve", 100, true),
    ];

    let cars = vec![
        ParkedCar::new(1, "Aston Martin Valkyrie", 1.0),
        ParkedCar::new(2, "McLaren P1", 1.0),
        ParkedCar::new(3, "Ferrari LaFerrari", 1.0),
        ParkedCar::new(4, "Porsche 918 Spyder", 1.0),
    ];
    let t = Track::new(track);
    let r = Race::new(t, 1, cars);
    let start = Instant::now();
    let standings = r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(700) + THRESHOLD;
    let lower_bound = Duration::from_millis(700) - THRESHOLD;

    assert!(
        standings
            .iter()
            .zip(vec![
                Duration::from_millis(400),
                Duration::from_millis(500),
                Duration::from_millis(600),
                Duration::from_millis(700)
            ])
            .map(|(cp, t)| (cp.global_time, t))
            .all(|(t_real, t_expected)| t_real <= t_expected + THRESHOLD
                && t_real >= t_expected - THRESHOLD)
    );
    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
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
    let t = Track::new(track);
    let r = Race::new(t, 3, cars);
    let start = Instant::now();
    r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(1200) + THRESHOLD;
    let lower_bound = Duration::from_millis(1200) - THRESHOLD;

    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}

#[test]
fn single_car_nonexclusive() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "Wide Straight", 100, false),
        TrackSector::new(2, "Literally a Landing Strip", 100, false),
        TrackSector::new(3, "Straight", 100, false),
        TrackSector::new(4, "Wide Straight", 100, false),
    ];

    let cars = vec![ParkedCar::new(1, "Aston Martin Valkyrie", 1.0)];
    let t = Track::new(track);
    let r = Race::new(t, 3, cars);
    let start = Instant::now();
    r.start_race();
    let elapsed = start.elapsed();

    let upper_bound = Duration::from_millis(1200) + THRESHOLD;
    let lower_bound = Duration::from_millis(1200) - THRESHOLD;
    println!("{:?}", elapsed);
    assert!(elapsed >= lower_bound && elapsed <= upper_bound);
}

#[test]
fn single_sector_many_cars_nonexclusive() {
    let track: Vec<TrackSector> = vec![TrackSector::new(1, "Sector", 1000, false)];

    let cars = vec![
        ParkedCar::new(1, "Car", 1.0),
        ParkedCar::new(2, "Car", 1.0),
        ParkedCar::new(3, "Car", 1.0),
        ParkedCar::new(4, "Car", 1.0),
        ParkedCar::new(5, "Car", 1.0),
        ParkedCar::new(6, "Car", 1.0),
        ParkedCar::new(7, "Car", 1.0),
        ParkedCar::new(8, "Car", 1.0),
        ParkedCar::new(9, "Car", 1.0),
        ParkedCar::new(10, "Car", 1.0),
        ParkedCar::new(11, "Car", 1.0),
        ParkedCar::new(12, "Car", 1.0),
        ParkedCar::new(13, "Car", 1.0),
        ParkedCar::new(14, "Car", 1.0),
        ParkedCar::new(15, "Car", 1.0),
        ParkedCar::new(16, "Car", 1.0),
        ParkedCar::new(17, "Car", 1.0),
        ParkedCar::new(18, "Car", 1.0),
        ParkedCar::new(19, "Car", 1.0),
        ParkedCar::new(20, "Car", 1.0),
        ParkedCar::new(21, "Car", 1.0),
        ParkedCar::new(22, "Car", 1.0),
        ParkedCar::new(23, "Car", 1.0),
        ParkedCar::new(24, "Car", 1.0),
        ParkedCar::new(25, "Car", 1.0),
        ParkedCar::new(26, "Car", 1.0),
        ParkedCar::new(27, "Car", 1.0),
        ParkedCar::new(28, "Car", 1.0),
        ParkedCar::new(29, "Car", 1.0),
        ParkedCar::new(30, "Car", 1.0),
        ParkedCar::new(31, "Car", 1.0),
        ParkedCar::new(32, "Car", 1.0),
        ParkedCar::new(33, "Car", 1.0),
        ParkedCar::new(34, "Car", 1.0),
        ParkedCar::new(35, "Car", 1.0),
        ParkedCar::new(36, "Car", 1.0),
        ParkedCar::new(37, "Car", 1.0),
        ParkedCar::new(38, "Car", 1.0),
        ParkedCar::new(39, "Car", 1.0),
        ParkedCar::new(40, "Car", 1.0),
        ParkedCar::new(41, "Car", 1.0),
        ParkedCar::new(42, "Car", 1.0),
        ParkedCar::new(43, "Car", 1.0),
        ParkedCar::new(44, "Car", 1.0),
        ParkedCar::new(45, "Car", 1.0),
        ParkedCar::new(46, "Car", 1.0),
        ParkedCar::new(47, "Car", 1.0),
        ParkedCar::new(48, "Car", 1.0),
        ParkedCar::new(49, "Car", 1.0),
        ParkedCar::new(50, "Car", 1.0),
    ];
    let t = Track::new(track);
    let r = Race::new(t, 1, cars);
    let standings = r.start_race();

    match (standings.first(), standings.last()) {
        (Some(first), Some(last)) => assert!(last.global_time - first.global_time <= THRESHOLD / 8),
        _ => assert!(false),
    }
}
