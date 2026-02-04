use crate::race::Num;
use crate::track::Track;
use crate::utils::analytics::{SectorInfo, Stat};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Barrier};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Struct used for initialization of cars.
/// This struct specifies basic information about cars
pub struct ParkedCar {
    pub id: Num,
    pub name: String,
    /// A single numerical value encompassing all aspects of a car's performance
    /// (for simplicity's sake).
    /// Higher value means better performance.
    pub performance: f64,
}

impl ParkedCar {
    pub fn new(id: Num, name: &str, performance: f64) -> Self {
        Self {
            id,
            name: String::from(name),
            performance,
        }
    }
}

/// Actual car racing on a track
/// Synchronization primitives and a global stopwatch were added
#[derive(Clone)]
pub struct Car {
    pub id: Num,
    pub name: String,
    pub performance: f64,
    pub track: Arc<Track>,
    pub stopwatch: Arc<Instant>,
    pub start_barrier: Arc<Barrier>,
}

impl Car {
    pub fn new(
        id: Num,
        name: String,
        performance: f64,
        track: Arc<Track>,
        stopwatch: Arc<Instant>,
        start_barrier: Arc<Barrier>,
    ) -> Self {
        Self {
            id,
            name,
            performance,
            track,
            stopwatch,
            start_barrier,
        }
    }

    /// A method that converts a parked car into an on-track car.
    /// Clones synchronization primitives provided
    pub fn from_parked_car(
        car: ParkedCar,
        track: &Arc<Track>,
        stopwatch: &Arc<Instant>,
        start_barrier: &Arc<Barrier>,
    ) -> Self {
        Self {
            id: car.id,
            name: car.name,
            performance: car.performance,
            track: track.clone(),
            stopwatch: stopwatch.clone(),
            start_barrier: start_barrier.clone(),
        }
    }

    /// Simulates a single lap.
    /// After passing a sector, corresponding sector time
    /// should be transmitted over a channel to the race_analysis() method defined on the Race struct.
    /// The Stat::new_sector_stat() function is used to create structs
    /// that can be transmitted over the channel.
    fn simulate_lap(&self, lap: Num, tx: &Sender<Stat>) {
        let mut cars_waited = false;
        let mut is_safety_car = self.track.safety_car_deployed.load(Ordering::Relaxed);
        for sector in self.track.sectors.iter() {
            if !is_safety_car {
                //we check this only if it is false, once it is true, we will never change it back to false and finish the lap with safety car
                is_safety_car = self.track.safety_car_deployed.load(Ordering::Relaxed);
                if is_safety_car && !cars_waited {
                    self.start_barrier.wait();
                    cars_waited = true;
                }
            }

            let _guard = if sector.exclusive {
                //use idiomatic apporach since rust doesnt
                // have unlock() or isnt recommended to use drop(), so i added this {} scope definition
                Some(sector.exclusive_lock.lock().unwrap())
            } else {
                None
            };

            let duration = self.get_duration(sector.time, is_safety_car); //here i use the same code for both exclusive and non exclusive  since it was duplicite code and not idiomatic:D!
            sleep(duration);
            let sector_info = SectorInfo::new(&sector.name, sector.order, duration, is_safety_car);
            let stopwatch_time = self.stopwatch.elapsed();

            let stat = Stat::new_sector_stat(self.id, &self.name, stopwatch_time, lap, sector_info);
            tx.send(stat).unwrap();
        } //here automatic unlock out of the scope, hopefully
        if self.track.safety_car_deployed.load(Ordering::Relaxed) {
            self.track
                .safety_car_deployed
                .store(false, Ordering::Relaxed); //Note: i had barrier here, but i delayed racers bcs all waited to write it at the same time..wrong apporach
        }
    }

    /// Simulates the entire race for this car.
    /// Lap times are directly measured here using the car's global clock
    /// should be transmitted over a channel to the race_analysis() method defined on the Race struct.
    /// The Stat::new_lap_stat() function is used to create structs
    /// that can be transmitted over the channel
    pub fn simulate_race(&self, laps: Num, tx: Sender<Stat>) {
        self.start_barrier.wait(); //Make sure to wait for all cars to initialize before setting them off.
        for lap in 1..=laps {
            //Start iterating from lap number 1.
            let lap_time_start = self.stopwatch.elapsed();
            self.simulate_lap(lap, &tx);
            let lap_time_end = self.stopwatch.elapsed();
            let duration_of_lap = lap_time_end - lap_time_start;

            let stat = Stat::new_lap_stat(
                self.id,
                &self.name,
                self.stopwatch.elapsed(),
                lap,
                duration_of_lap,
            );
            tx.send(stat).unwrap(); //Transmit the value returned by Stat::new_lap_stat() via the provided tx handle of the channel after completing a lap.
        }
    }

    /// Helper function that calculates correct sector times adjusted for the car's performance
    /// and the Safety Car, if deployed.
    fn get_duration(&self, sector_time: u32, safety_car: bool) -> Duration {
        let mut time = f64::from(sector_time) / self.performance;
        if safety_car {
            time /= self.track.safety_car_reduction.unwrap_or(1_f64);
        }
        Duration::from_millis(time.round() as u64)
    }
}
