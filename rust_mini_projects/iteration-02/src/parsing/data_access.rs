use std::collections::BTreeMap;

use serde::de::DeserializeOwned;

use crate::parsing::datasets::basic_settlement_units::BasicSettlementUnit;
use crate::parsing::datasets::lamps::Lamp;
use crate::parsing::datasets::traffic_accident::{TrafficAccident, TrafficAccidentRow, Vehicle};

pub struct DataAccess {}

impl DataAccess {
    const UNITS_PATH: &'static str = "./data/basic_settlement_units.csv";
    const ACCIDENTS_PATH: &'static str = "./data/traffic_accidents.csv";
    const LAMPS_PATH: &'static str = "./data/lampy.csv";

    fn read_and_parse_to_vec<T: DeserializeOwned>(data_path: &str) -> Vec<T> {
        let mut reader = csv::Reader::from_path(data_path).unwrap();
        let mut values = Vec::new();

        for row in reader.deserialize() {
            let value: T = row.unwrap();
            values.push(value);
        }

        values
    }

    fn read_accidents(data_path: &str) -> Vec<TrafficAccident> {
        let mut reader = csv::ReaderBuilder::new()
            .buffer_capacity(8 * (1 << 20))
            .from_path(data_path)
            .unwrap();

        reader
            .deserialize()
            .fold(BTreeMap::new(), |mut acc, item| {
                let row: TrafficAccidentRow = item.unwrap();
                let is_pedestrian = row.is_pedestrian();
                let is_occupant = row.is_occupant();
                let (overview, mut vehicle, pedestrian, occupant) = row.granulate();

                let entry = acc
                    .entry(overview.police_accident_id)
                    .or_insert(TrafficAccident {
                        overview,
                        participant_vehicles: vec![],
                        participant_pedestrians: vec![],
                    });

                if is_pedestrian {
                    entry.participant_pedestrians.push(pedestrian);
                } else {
                    let vehicle_entry: Option<&mut Vehicle> =
                        entry.participant_vehicles.iter_mut().find(|v| {
                            v.vehicle_id_within_accident == vehicle.vehicle_id_within_accident
                        });

                    match vehicle_entry {
                        None => {
                            if is_occupant {
                                vehicle.occupants.push(occupant);
                            }
                            entry.participant_vehicles.push(vehicle);
                        }
                        Some(&mut ref mut v) => {
                            if is_occupant {
                                v.occupants.push(occupant);
                            }
                        }
                    };
                }
                acc
            })
            .into_values()
            .collect::<Vec<_>>()
    }

    pub fn basic_settlement_units() -> Vec<BasicSettlementUnit> {
        DataAccess::read_and_parse_to_vec(DataAccess::UNITS_PATH)
    }
    pub fn lamps() -> Vec<Lamp> {
        DataAccess::read_and_parse_to_vec(DataAccess::LAMPS_PATH)
    }

    pub fn accidents() -> Vec<TrafficAccident> {
        Self::read_accidents(Self::ACCIDENTS_PATH)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader, Lines, Result};

    use crate::parsing::data_access::DataAccess;
    use crate::parsing::datasets::traffic_accident::TrafficAccidentRow;

    fn file_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
        let file = match File::open(file_path) {
            Err(why) => return Err(why),
            Ok(file) => file,
        };

        Ok(BufReader::new(file).lines())
    }

    #[test]
    fn test_units_csv_parsing_ok() -> Result<()> {
        let values = DataAccess::basic_settlement_units();
        let file_lines = file_lines(DataAccess::UNITS_PATH)?;

        let expected = file_lines.count() - 1; // minus the header line
        let actual = values.len();
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_traffic_accidents_csv_parsing_ok() -> Result<()> {
        let values: Vec<TrafficAccidentRow> =
            DataAccess::read_and_parse_to_vec(DataAccess::ACCIDENTS_PATH);
        let file_lines = file_lines(DataAccess::ACCIDENTS_PATH)?;

        let expected = file_lines.count() - 1; // minus the header line
        let actual = values.len();
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_traffic_accidents_accident_ids_ok() -> Result<()> {
        let values = DataAccess::accidents();

        let mut ids = file_lines(DataAccess::ACCIDENTS_PATH)?
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|line| {
                let id = line.split(',').nth(1).map(|inner| inner.to_string());

                id.ok_or(io::Error::from(io::ErrorKind::InvalidData))
            })
            .collect::<Result<Vec<_>>>()?;
        ids.sort();
        ids.dedup();

        let expected = ids.len() - 1; // minus the header line;
        let actual_accident_ids = values.len();

        assert_eq!(expected, actual_accident_ids);
        assert_eq!(expected, 32908);

        Ok(())
    }

    #[test]
    fn test_traffic_accidents_participants_ok() -> Result<()> {
        let values = DataAccess::accidents();
        let actual_rows = values.iter().fold(0, |acc, accident| {
            acc + accident
                .participant_vehicles
                .iter()
                .fold(0, |acc2, vehicle| {
                    acc2 + usize::max(1, vehicle.occupants.len())
                })
                + accident.participant_pedestrians.len()
        });

        let file_lines = file_lines(DataAccess::ACCIDENTS_PATH)?;
        let expected_rows = file_lines.count() - 1; // minus the header line

        assert_eq!(expected_rows, actual_rows);
        assert_eq!(expected_rows, 69740);

        Ok(())
    }
}
