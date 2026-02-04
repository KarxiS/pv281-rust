use eboladrome::car::ParkedCar;
use eboladrome::race::Race;
use eboladrome::track::{Track, TrackSector};

fn main() {
    let track: Vec<TrackSector> = vec![
        TrackSector::new(1, "The Isn't Straight", 900, false),
        TrackSector::new(2, "Your Name Here", 300, true),
        TrackSector::new(3, "The Isn't Straight", 900, false),
        TrackSector::new(4, "Old Lady's House", 400, true),
        TrackSector::new(5, "Bumpy Back Straight", 300, false),
        TrackSector::new(6, "Substation", 40, true),
        TrackSector::new(7, "Field Of Sheep", 45, true),
    ];

    let cars = vec![
        ParkedCar::new(1, "McLaren P1", 1.0),
        ParkedCar::new(2, "Ferrari LaFerrari", 1.0),
        ParkedCar::new(3, "Porsche 918 Spyder", 1.0),
        ParkedCar::new(4, "Reasonable car", 0.4),
    ];

    let t = Track::new_with_safety_car(track, 0.5);
    let r = Race::new(t, 5, cars);
    let standings = r.start_race();
    for (i, c) in standings.iter().enumerate() {
        println!(
            "{}. {} finished in {:.4} seconds",
            i + 1,
            c.name,
            c.global_time.as_secs_f64()
        );
    }
}
