use it02_data_processing::tasks::most_frequent_type_of_light;

#[test]
fn test_most_frequent_type_of_light_ok() {
    let expected =
        "Najcastejsi typ :V-Siteco Pilz 50W Opál, prvy krat instalovany : 1975/01/01 00:00:00+00";
    let actual = most_frequent_type_of_light();

    assert_eq!(expected, actual);
}
