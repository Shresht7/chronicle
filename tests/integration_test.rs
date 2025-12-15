use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_snapshot_command_adds_files() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create some dummy files in the temporary directory
    std::fs::write(dir_path.join("file1.txt"), "hello world")?;
    std::fs::write(dir_path.join("file2.txt"), "another file")?;

    // Run the chronicle snapshot command
    let output = Command::new(env!("CARGO_BIN_EXE_chronicle"))
        .arg("snapshot")
        .arg(dir_path)
        .output()?;

    // Assert that the command exited successfully
    assert!(output.status.success());

    // Convert stdout and stderr to strings
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Print stdout and stderr for debugging in case of failure
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Assert that the output contains expected messages
    assert!(stdout.contains("Snapshot detected changes:"));
    assert!(stdout.contains("+ 2 added files"));
    assert!(stdout.contains("Snapshot stored with id"));

    Ok(())
}
