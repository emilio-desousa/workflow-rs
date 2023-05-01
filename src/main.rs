use std::process::Command;
use std::process::ExitStatus;

use clap::Parser;

#[derive(Parser)]
struct PythonScript {
    python_file_path: String,
    python_env_binary_path: String,
}

impl PythonScript {
    fn run(&self) -> ExitStatus {
        let output = Command::new(&self.python_env_binary_path)
            .arg(&self.python_file_path)
            .output()
            .expect("failed to execute process");

        println!("Execution done.");
        println!("Status code is: \n{}", output.status);
        output.status
    }
}

fn main() -> ExitStatus {
    let python_script = PythonScript::parse();
    python_script.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_script_run_success() {
        let python_script = PythonScript {
            python_file_path: "tests/success.py".to_string(),
            python_env_binary_path: "/usr/bin/python3".to_string(),
        };

        let return_code = python_script.run();
        const SUCCESS_CODE: i32 = 0;

        assert_eq!(return_code.code(), Some(SUCCESS_CODE));
    }
    #[test]
    fn test_python_script_run_failing() {
        let python_script = PythonScript {
            python_file_path: "tests/error.py".to_string(),
            python_env_binary_path: "/usr/bin/python3".to_string(),
        };

        let return_code = python_script.run();
        const FAILING_CODE: i32 = 1;

        assert_eq!(return_code.code(), Some(FAILING_CODE));
    }
}
