use crate::parsing::data_access::DataAccess;
use itertools::Itertools;

/// Find and format accidents that are somehow extreme:
/// the damages were the highest,
/// the accident involved the most people,
/// the accident involved the most vehicles.
/// Are at least two of them the same accident?
pub fn extreme_accidents() -> (String, bool) {
    let data = DataAccess::accidents();
    //max dmg in czk
    let max_damage = data
        .iter()
        .map(|x| (&x.overview.global_id, x.overview.damage_in_czk))
        .max_by_key(|(_, damage)| *damage);
    //max vehicles taken from length of vector participant_vehicles
    let max_vehicles = data
        .iter()
        .map(|x| (&x.overview.global_id, x.participant_vehicles.len()))
        .max_by_key(|(_, v)| *v);
    //opening up participant_vehicles of one traffic accident for all vehicles... then counting them up via another map and sum, then call max_by_key to get maximum of occupants on traffic accident
    let max_occupants = data
        .iter()
        .map(|x| {
            let no_occupants: usize = x
                .participant_vehicles
                .iter()
                .map(|vehicle| vehicle.occupants.len())
                .sum();

            (&x.overview.global_id, no_occupants)
        })
        .max_by_key(|(_, no_occupants)| *no_occupants);
    //unwrap because i will always find some data if i have at least 1 datapoint
    let ids = [
        max_damage.unwrap().0,
        max_vehicles.unwrap().0,
        max_occupants.unwrap().0,
    ];
    let unique_count = ids.iter().unique().count();
    let has_duplicate = unique_count < 3; //if i have less than 3 uniques, then it means i have at least one duplication of global unique ID

    let damage_value = max_damage.unwrap().1;
    let vehicles_count = max_vehicles.unwrap().1;
    let occupants_count = max_occupants.unwrap().1;

    let string_build = format!(
        "{} Kč : {} vehicles : {} people",
        damage_value, vehicles_count, occupants_count
    );

    (string_build, has_duplicate)
}
