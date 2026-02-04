use crate::parsing::data_access::DataAccess;
use crate::parsing::datasets::traffic_accident::enums::PersonDetail::{
    AirbagDeployedWhileNotWearingASeatbelt, NotWearingASeatbelt,
};
use chrono::Datelike;
use itertools::Itertools;
///enum yeartrend - positive, negative, neutral, error
enum YearTrends {
    Positive,
    Negative,
    Neutral,
    Error,
}
///char representation for yeartrends
impl YearTrends {
    fn to_char(&self) -> char {
        match self {
            YearTrends::Positive => '^',
            YearTrends::Negative => 'v',
            YearTrends::Neutral => '=',
            YearTrends::Error => 'X',
        }
    }
}

/// Encode the trend of accidents with unbuckled vehicle occupants over year-long windows into a String.
pub fn unbuckled_trend() -> String {
    let data = DataAccess::accidents();
    let mut occupants: Vec<(i32, usize)> = data
        .iter()
        .map(|accident| {
            let year = accident.overview.date.year();

            let no_seatbelt = accident
                .participant_vehicles
                .iter()
                .flat_map(|vehicle| &vehicle.occupants)
                .filter(|occupant| {
                    matches!(
                        occupant.person_detail,
                        Some(NotWearingASeatbelt) | Some(AirbagDeployedWhileNotWearingASeatbelt)
                    )
                })
                .count();

            (year, no_seatbelt)
        })
        .collect();
    occupants.sort_by_key(|(year, _)| *year);
    let group_by_year = occupants.iter().chunk_by(|(year, _)| year);
    let year_sums = group_by_year.into_iter().map(|(year, group)| {
        let sum: usize = group.map(|(_, number)| number).sum();
        (year, sum)
    });

    let mut actual_year = year_sums.into_iter();
    let prev_year = actual_year.next();
    let trends = actual_year
        .scan(prev_year, |state, (year, sum)| {
            //scan remembers the previous state , couldnt do in map - ideal for previous-actual comparsion
            let trend = match (state.unwrap().1, sum) {
                (p, c) if p > c => YearTrends::Negative,
                (p, c) if p < c => YearTrends::Positive,
                (p, c) if p == c => YearTrends::Neutral,
                _ => YearTrends::Error,
            };
            *state = Some((year, sum));
            Some(trend)
        })
        .collect::<Vec<_>>();
    //convert to chars then collect it to get collection of whole string
    trends
        .iter()
        .map(|trend| trend.to_char())
        .collect::<String>()
}
