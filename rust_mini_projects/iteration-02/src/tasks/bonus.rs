use crate::parsing::data_access::DataAccess;
use itertools::Itertools;

///write out the most used type of light and also write out the time it was first installed.
pub fn most_frequent_type_of_light() -> String {
    let data = DataAccess::lamps();
    let data_extracted: Vec<_> = data
        .iter()
        .map(|x| (&x.type_of_light, &x.date_of_install))
        .collect();
    let counts = data_extracted
        .iter()
        .counts_by(|(type_of_light, _)| type_of_light);
    let (most_frequent, _) = counts.iter().max_by_key(|(_, count)| *count).unwrap();

    let first_installed = data_extracted
        .iter()
        .filter(|(name, _)| name == *most_frequent)
        .map(|(_, time_of_install)| time_of_install)
        .min()
        .unwrap();

    format!(
        "Najcastejsi typ :{}, prvy krat instalovany : {}",
        most_frequent, first_installed
    )
}
