use std::process::Command;

use clap::Parser;

#[derive(Parser)]
struct PythonScript {
    python_file_path: String,
    python_env_binary_path: String,
}

impl PythonScript {
    fn run(&self) {
        let output = Command::new(&self.python_env_binary_path)
            .arg(&self.python_file_path)
            .output()
            .expect("failed to execute process");

        println!("Execution done.");
        println!("Status code is: \n{}", output.status)
    }
}

fn main() {
    let python_script = PythonScript::parse();
    python_script.run()
}
