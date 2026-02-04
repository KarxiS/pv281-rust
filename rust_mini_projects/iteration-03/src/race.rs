use crate::car::{Car, ParkedCar};
use crate::track::Track;
use crate::utils::analytics::{CarPosition, Stat, StatDetail, print_board};
use crate::utils::ordered_map::OrderedMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::Instant;

pub type Num = u64;

/// Struct representing a race.
pub struct Race {
    track: Arc<Track>,
    cars: Vec<Car>,
    laps: Num,
}

impl Race {
    pub fn new(track: Track, laps: Num, cars: Vec<ParkedCar>) -> Self {
        let track = Arc::new(track);
        Self {
            cars: Self::construct_cars(&track, cars),
            track,
            laps,
        }
    }

    /// Function that converts parked cars into on-track cars with a stopwatch
    /// and appropriate sync primitives
    fn construct_cars(track: &Arc<Track>, cars: Vec<ParkedCar>) -> Vec<Car> {
        let car_count = cars.len();
        let stopwatch = Arc::new(Instant::now());
        let start_barrier = Arc::new(Barrier::new(car_count)); //Make sure to initialize the field in the construct_cars() function.
        cars.into_iter()
            .map(|pc| Car::from_parked_car(pc, track, &stopwatch, &start_barrier))
            .collect()
    }

    /// Starts a race spawning multiple threads for various purposes
    /// Started threads should include at least these threads:
    /// - A thread triggering the safety car
    /// - A thread executing the race_analysis() method
    /// - A thread per each car executing the car's simulate_race() method.
    ///
    /// Scoped threads or ordinary thread::spawn can be used
    ///
    /// Returns a vector of CarPositions representing the final standing of cars after
    /// crossing the finish line, these results are returned from the race_analysis() method
    pub fn start_race(&self) -> Vec<CarPosition> {
        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            let race_anaylsis = s.spawn(move || {
                //i call move because rust doesnt allow rx to be shared so i send ownership it to the scope here
                self.race_analysis(&rx)
            });
            s.spawn(|| {
                self.track.release_safety_car(self.laps);
            });

            for car in self.cars.iter() {
                let car_clone = car.clone();
                let tx = tx.clone();
                s.spawn(move || {
                    car_clone.simulate_race(self.laps, tx); //need to call clone because of error of loop ownesship steal
                });
            }
            drop(tx);
            race_anaylsis.join().unwrap()
        })
    }

    /// Analyzes the race, replacing a car's position on the timing board
    /// with a new one.
    /// CarPosition is constructed from a Stat received from the channel.
    /// Returns a vector of car positions representing the final standing of cars after the race.
    pub fn race_analysis(&self, rx: &Receiver<Stat>) -> Vec<CarPosition> {
        let mut board: OrderedMap<Num, CarPosition> = OrderedMap::new();
        for car in self.cars.iter() {
            board.insert(car.id, CarPosition::new_empty(car.id, car.name.clone()));
        }
        for stat in rx {
            // if the obtained stat does not contain some information, old CarPosition is used
            let old = board.get(&stat.car_id).unwrap_or_default();

            // for example, if a stat is of type LapDetail, it does not contain sector info,
            // so it would become blank, for that reason it is replaced by the old sector_info
            // and only the lap time is updated
            match stat.stat_detail {
                StatDetail::LapDetail(lap_time) => {
                    board.replace(
                        stat.car_id,
                        CarPosition::new(
                            stat.car_id,
                            stat.car_name,
                            stat.global_time,
                            stat.lap,
                            lap_time,
                            old.sector_info.clone(),
                        ),
                    );
                }
                StatDetail::SectorDetail(sector) => {
                    board.replace(
                        stat.car_id,
                        CarPosition::new(
                            stat.car_id,
                            stat.car_name,
                            stat.global_time,
                            stat.lap,
                            old.lap_time,
                            sector,
                        ),
                    );
                }
            }
            print_board(&board);
        }
        board.into_iter().map(|p| (*p).clone()).collect()
    }
}
