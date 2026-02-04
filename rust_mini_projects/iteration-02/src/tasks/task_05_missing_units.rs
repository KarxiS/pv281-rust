use crate::parsing::data_access::DataAccess;
use std::collections::HashSet;

/// Which basic settlement units in Brno have never had a recorded accident?
pub fn missing_units() -> HashSet<String> {
    let accidents = DataAccess::accidents();
    let bsu_data = DataAccess::basic_settlement_units();
    //because of hashset- creating new structure i need to call clone - cant use references - different from previous tasks!
    let brno_bsu: HashSet<String> = bsu_data
        .iter()
        .filter(|x| x.municipality_name == "Brno")
        .map(|x| x.name.clone())
        .collect();
    //because of hashset- creating new structure i need to call clone - cant use references - different from previous tasks!
    let accident_all_bsu: HashSet<String> = accidents
        .iter()
        .map(|x| x.overview.basic_settlement_unit.clone())
        .collect();
    //need to call cloned, otherwise same thing happens- references will get lost of accidents from hashsets
    brno_bsu.difference(&accident_all_bsu).cloned().collect()
}
