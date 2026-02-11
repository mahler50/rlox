use std::{fs::File, io::Read, process::Command};

use walkdir::WalkDir;

#[test]
fn lox_test() {
    for (input, output) in get_testcase_file_pair() {
        eprintln!("Testing {:?}!", input);
        match run_testcases(&input) {
            Ok(stdout) => {
                let norm = |s: &str| s.trim_end().replace("\r\n", "\n");
                let mut expected = String::new();
                File::open(&output)
                    .and_then(|mut file| file.read_to_string(&mut expected))
                    .expect("Failed to read output");
                assert_eq!(
                    norm(&stdout),
                    norm(&expected),
                    "Mismatch output for input file: {}",
                    input
                );
            }
            Err(stderr) => {
                eprintln!("Error running test: {}", stderr);
                panic!("Failed to running test for input file: {}", input);
            }
        }
    }
}

fn get_testcase_file_pair() -> Vec<(String, String)> {
    WalkDir::new("testcases")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "lox"))
        .map(|entry| {
            let input = entry.into_path();
            let output = input.with_extension("txt");
            (
                input.to_string_lossy().to_string(),
                output.to_string_lossy().to_string(),
            )
        })
        .collect()
}

fn run_testcases(path: &str) -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", path])
        .output()
        .expect("Failed to run test");

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
