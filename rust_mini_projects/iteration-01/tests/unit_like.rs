use assert_cmd::Command;
use std::error::Error;

#[test]
fn test_no_argument_fails() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_superfluous_argument_succeeds() -> Result<(), Box<dyn Error>> {
    // Check that an empty sourcefile will succeed
    test_empty_succeeds_output()?;

    // Assume success on empty sourcefile, test the two arguments situation
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .args([
            "./tests/files/zipffucks/empty.zf",
            "./tests/files/zipffucks/empty.zf",
        ])
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_a_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_stdout = "a\n";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/a.zf")
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_a_with_comments_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_stdout = "a\n";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/a_with_comments.zf")
        .assert()
        .success()
        .stdout(expected_stdout);

    Ok(())
}

#[test]
fn test_empty_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_output = "";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/empty.zf")
        .assert()
        .success()
        .stdout(expected_output)
        .stderr(expected_output);

    Ok(())
}

#[test]
fn test_loop_startless_fails() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/loop_startless.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_loop_endless_fails() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/loop_endless.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_loop_flipped_fails() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/loop_flipped.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_cell_size_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_output = "8 bit cells\n";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/cell_size.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .stdout(expected_output);

    Ok(())
}

#[test]
fn test_memory_size_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_output_size = 30000;

    let actual_output_size = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/memory_size.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .get_output()
        .stdout
        .len();

    assert_eq!(actual_output_size, expected_output_size);

    Ok(())
}

#[test]
fn test_bounds_left_succeeds() -> Result<(), Box<dyn Error>> {
    let expected_output = "";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/bounds_left.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .stdout(expected_output)
        .stderr(expected_output);

    Ok(())
}

#[test]
fn test_bounds_right_succeeds() -> Result<(), Box<dyn Error>> {
    let expected_output = "";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/bounds_right.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .stdout(expected_output)
        .stderr(expected_output);

    Ok(())
}

#[test]
fn test_echo_char_succeeds() -> Result<(), Box<dyn Error>> {
    for c in b' '..=b'~' {
        // has to be turned into a Vec<u8>
        let input = vec![c];
        let expected_output = input.clone();

        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .arg("./tests/files/zipffucks/echo_char.zf")
            .timeout(std::time::Duration::from_secs(5))
            .write_stdin(input)
            .assert()
            .success()
            .stdout(expected_output);
    }

    Ok(())
}

#[test]
fn test_h_succeeds_output() -> Result<(), Box<dyn Error>> {
    let expected_output = "H";

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("./tests/files/zipffucks/h.zf")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .stdout(expected_output);

    Ok(())
}
