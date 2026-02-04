use crate::race::Num;
use rand::{Rng, rng};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

/// Struct representing the track.
/// Stores track sectors and a flag signalling the safety car's deployment
/// and speed reduction per sector that incurs when the safety car is deployed
#[derive(Clone)]
pub struct Track {
    pub sectors: Vec<TrackSector>,
    // Lower value means higher speed reduction
    pub safety_car_reduction: Option<f64>,
    pub safety_car_deployed: Arc<AtomicBool>,
}

impl Track {
    pub fn new(track_sectors: Vec<TrackSector>) -> Self {
        Self {
            sectors: track_sectors,
            safety_car_reduction: None,
            safety_car_deployed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// An alternative function for obtaining a track (with a safety car)
    pub fn new_with_safety_car(track_sectors: Vec<TrackSector>, safety_car_reduction: f64) -> Self {
        Self {
            sectors: track_sectors,
            safety_car_reduction: Some(safety_car_reduction),
            safety_car_deployed: Arc::new(AtomicBool::new(false)), //added here false , because fn correct_standing()
                                                                   // was deadlocked here in with_safety_car.rs test , others working find though.
                                                                   // the reason is we dont start deployed, so setting it to true is wrong. We are setting it to true after some time we are going to be deployed(the sleep there that is prewritten from teacher)
        }
    }

    /// A function that is being executed in a separate thread
    /// The analyze_track function is used to estimate the lower and upper bounds of race's duration
    /// Then a random time is obtained from the estimated range using a random generator.
    /// Thread sleeps for given amount of time before releasing the safety car.
    pub fn release_safety_car(&self, laps: Num) {
        if self.safety_car_reduction.is_none() {
            return;
        }
        let delay = self.analyze_track(laps);
        let random_duration = rng().random_range(delay.0..delay.1);
        // Random wait time before releasing the safety car
        sleep(random_duration);
        self.safety_car_deployed.store(true, Ordering::Relaxed)
    }

    /// Analyzes the track and estimates lower and upper bounds of race's total duration
    /// (excluding lock contention on exclusive sectors and the impact of the safety car).
    fn analyze_track(&self, laps: Num) -> (Duration, Duration) {
        let track_length = self.sectors.iter().map(|s| s.time as Num).sum::<Num>();
        let lower_delay = Duration::from_millis(track_length);
        let upper_delay = Duration::from_millis(track_length * laps);
        (lower_delay, upper_delay)
    }
}

/// Struct representing a track sector.
/// Each sector has a flag indicating if given sector is exclusive, i.e.
/// only one car can pass the sector at any given time.
/// Add any fields to this struct that will allow you to implement the feature
///Task 1: Add a synchronization primitive for implementation of exclusive sectors.
#[derive(Clone)]
pub struct TrackSector {
    pub order: Num,
    pub name: String,
    pub time: u32,
    pub exclusive: bool,
    pub exclusive_lock: Arc<Mutex<()>>,
}

impl TrackSector {
    pub fn new(order: Num, name: &str, time: u32, exclusive: bool) -> Self {
        TrackSector {
            order,
            name: String::from(name),
            time,
            exclusive,
            exclusive_lock: Arc::new(Mutex::new(())),
        }
    }
}
