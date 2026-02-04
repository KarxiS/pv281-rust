use crate::parsing::data_access::DataAccess;
use crate::parsing::datasets::traffic_accident::enums::{AccidentTime, TimeOfDay};
use chrono::NaiveTime;

/// What range of timestamps do accidents which are considered to have happened during the night fall into?
pub fn night_times() -> (NaiveTime, NaiveTime) {
    let data = DataAccess::accidents();

    let nighttimedata: Vec<_> = data
        .iter()
        .filter(|x| x.overview.time_of_day == TimeOfDay::Nighttime)
        .filter(|x| matches!(x.overview.time, AccidentTime::Exact(_))) //i use this because filter with (_) didnt work and this was recommended to me to use
        .filter_map(|x| match x.overview.time {
            //unpack AccidentTime to get safe max, min
            AccidentTime::Exact(time) => Some(time),
            _ => None,
        })
        .collect();

    let (before_lunch, after_lunch): (Vec<&NaiveTime>, Vec<&NaiveTime>) = nighttimedata
        .iter() //using partition i cannot do Vec<_>
        .partition(|x| **x < NaiveTime::from_hms_opt(13, 00, 00).unwrap());
    //You can assume that there will be at least one accident in the dataset after midnight and before midnight with a known exact time.
    let start = *after_lunch.iter().min().unwrap(); //something between 20:00 - 23:59
    let end = *before_lunch.iter().max().unwrap(); //something after midnight
    (*start, *end)
}
