use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::process::ExitStatus;
#[derive(Debug, Deserialize)]
struct YamlScript {
    scripts: Vec<PythonScript>,
}
fn read_yaml(path: &str) -> Result<YamlScript, serde_yaml::Error> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let yaml: YamlScript = serde_yaml::from_str(&contents)?;
    Ok(yaml)
}

#[derive(Parser, Debug, Deserialize)]
struct PythonScript {
    script: String,
    python_binary: String,
}

impl PythonScript {
    fn run(&self) -> ExitStatus {
        let output = Command::new(&self.python_binary)
            .arg(&self.script)
            .output()
            .expect("failed to execute process");

        output.status
    }
}

fn main() -> Result<(), serde_yaml::Error> {
    let tmp = read_yaml("dummy_python_scripts/simple_yaml.yml")?;
    for script in tmp.scripts.iter() {
        println!("##########################");
        let status = script.run();
        match status.code() {
            Some(code) => match code {
                1 => panic!("Python process failed {:#?}", script),
                0 => println!("Ok good"),
                _ => println!("Not expected return code"),
            },
            None => print!("bla"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_script_run_success() {
        let python_script = PythonScript {
            script: "tests/success.py".to_string(),
            python_binary: "/usr/bin/python3".to_string(),
        };

        let return_code = python_script.run();
        const SUCCESS_CODE: i32 = 0;

        assert_eq!(return_code.code(), Some(SUCCESS_CODE));
    }
    #[test]
    fn test_python_script_run_failing() {
        let python_script = PythonScript {
            script: "tests/error.py".to_string(),
            python_binary: "/usr/bin/python3".to_string(),
        };

        let return_code = python_script.run();
        const FAILING_CODE: i32 = 1;

        assert_eq!(return_code.code(), Some(FAILING_CODE));
    }
}
