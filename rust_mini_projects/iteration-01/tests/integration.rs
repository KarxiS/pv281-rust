use std::error::Error;
use std::fs::read_to_string;

use assert_cmd::Command;

#[test]
fn test_99_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_stdout = read_to_string("./tests/files/expected/99.txt")?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/99.zf")
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_b_succeeds_output() -> Result<(), Box<dyn Error>> {
    // the input is encoded in the source file
    let expected_stdout = read_to_string("./tests/files/expected/b.txt")?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/b.zf")
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_cat_succeeds_output() -> Result<(), Box<dyn Error>> {
    let stdin_input = read_to_string("./tests/files/expected/cat.txt")?;
    let expected_stdout = stdin_input.clone();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/cat.zf")
        .write_stdin(stdin_input)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_sort_succeeds_output() -> Result<(), Box<dyn Error>> {
    let stdin_input = "Pack my box with five dozen liquor jugs.".to_string();

    let mut input_bytes = stdin_input.chars().collect::<Vec<_>>();
    input_bytes.sort();
    let expected_stdout = input_bytes.iter().collect::<String>();

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/sort.zf")
        .timeout(std::time::Duration::from_secs(5))
        .write_stdin(stdin_input)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_square_succeeds_output() -> Result<(), Box<dyn Error>> {
    for input_n in 0u8..=15 {
        let expected_stdout = (input_n * input_n).to_string();

        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .arg("./tests/files/zipffucks/square.zf")
            .write_stdin(vec![input_n])
            .assert()
            .success()
            .stdout(expected_stdout);
    }

    Ok(())
}

#[test]
fn test_seven_segment_succeeds_output() -> Result<(), Box<dyn Error>> {
    let input = "123456789";
    let expected_stdout = read_to_string("./tests/files/expected/seven_segment.txt")?;

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/seven_segment.zf")
        .timeout(std::time::Duration::from_secs(5))
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_wc_succeeds_output() -> Result<(), Box<dyn Error>> {
    let input =
        "Tough times never last.\nOnly tough people last.\nGlarbleloobywooohooglorksppppppss.";
    let expected_stdout = format!(
        "\t{}\t{}\t{}\n",
        input.matches('\n').count(),
        input.replace('\n', " ").matches(' ').count() + 1,
        input.chars().count()
    );

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/wc.zf")
        .timeout(std::time::Duration::from_secs(5))
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}
