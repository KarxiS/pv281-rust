use crate::race::Num;
use crate::utils::ordered_map::OrderedMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::time::Duration;

#[derive(Clone)]
pub enum StatDetail {
    LapDetail(Duration),
    SectorDetail(SectorInfo),
}

/// Structure representing a single stat that is passed down from cars to the race_analysis() method
#[derive(Clone)]
pub struct Stat {
    pub car_id: Num,
    pub car_name: String,
    pub global_time: Duration,
    pub lap: Num,
    pub stat_detail: StatDetail,
}

impl Stat {
    pub fn new_sector_stat(
        car_id: Num,
        car_name: &str,
        global_time: Duration,
        lap: Num,
        sector_info: SectorInfo,
    ) -> Self {
        Self {
            car_id,
            car_name: String::from(car_name),
            global_time,
            lap,
            stat_detail: StatDetail::SectorDetail(sector_info),
        }
    }
    pub fn new_lap_stat(
        car_id: Num,
        car_name: &str,
        global_time: Duration,
        lap: Num,
        lap_time: Duration,
    ) -> Self {
        Self {
            car_id,
            car_name: String::from(car_name),
            global_time,
            lap,
            stat_detail: StatDetail::LapDetail(lap_time),
        }
    }
}

#[derive(Clone, Default)]
pub struct SectorInfo {
    pub name: String,
    pub order: Num,
    pub time: Duration,
    pub safety_car: bool,
}

impl SectorInfo {
    pub fn new(name: &str, order: Num, sector_time: Duration, safety_car: bool) -> Self {
        Self {
            name: String::from(name),
            order,
            time: sector_time,
            safety_car,
        }
    }
}

/// Struct representing a car's position on the timing board.
#[derive(Clone, Default)]
pub struct CarPosition {
    pub id: Num,
    pub name: String,
    pub global_time: Duration,
    pub lap: Num,
    pub lap_time: Duration,
    pub sector_info: SectorInfo,
}

impl CarPosition {
    pub fn new_empty(id: Num, name: String) -> Self {
        Self {
            id,
            name,
            global_time: Duration::default(),
            lap: 1,
            lap_time: Duration::default(),
            sector_info: SectorInfo::default(),
        }
    }

    pub fn new(
        id: Num,
        name: String,
        global_time: Duration,
        lap: Num,
        lap_info: Duration,
        sector_info: SectorInfo,
    ) -> Self {
        Self {
            id,
            name,
            global_time,
            lap,
            lap_time: lap_info,
            sector_info,
        }
    }
}

impl Eq for CarPosition {}

impl PartialEq<Self> for CarPosition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for CarPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// CarPositions are ordered by lap, sector's order and if all previous values are equal
/// then by global time.
/// Other values like sector time and lap time proved to be unreliable,
/// But are still present in the ordering, since global time should be always unique
/// these are not expected to be used in the comparison.
impl Ord for CarPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lap != other.lap {
            return other.lap.cmp(&self.lap);
        }
        if self.sector_info.order != other.sector_info.order {
            return other.sector_info.order.cmp(&self.sector_info.order);
        }
        if self.global_time != other.global_time {
            return self.global_time.cmp(&other.global_time);
        }
        if self.sector_info.time != other.sector_info.time {
            return self.sector_info.time.cmp(&other.sector_info.time);
        }
        self.id.cmp(&other.id)
    }
}

impl Hash for CarPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Prints the timing board
pub fn print_board(board: &OrderedMap<Num, CarPosition>) {
    println!(
        "------------------------------------------------------------------------------------------------------------------------"
    );
    println!(
        "{0: <5} | {1: <25} | {2: <20} | {3: <10} | {4: <11} | {5: <11} | {6: <11} | {7: <10}",
        "Lap",
        "Car Name",
        "Sector Name",
        "Sector #",
        "Sector Time",
        "Lap Time",
        "Global Time",
        "Safety Car"
    );
    println!(
        "------------------------------------------------------------------------------------------------------------------------"
    );
    for pos in board.iter() {
        println!(
            "{0: <5} | {1: <25} | {2: <20} | {3: <10} | {4: <11.7} | {5: <11.7} | {6: <11.7} | {7: <10}",
            pos.lap,
            pos.name,
            pos.sector_info.name,
            pos.sector_info.order,
            pos.sector_info.time.as_secs_f64(),
            pos.lap_time.as_secs_f64(),
            pos.global_time.as_secs_f64(),
            pos.sector_info.safety_car
        );
    }

    println!();
}
