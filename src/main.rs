struct PythonScript {
    python_file_path: String,
    python_env_binary_path: String,
}

impl PythonScript {
    fn run(&self) {
        println!(
            "Running file {} with binary {}!",
            self.python_file_path, self.python_env_binary_path
        )
    }
}

fn main() {
    let python_script = PythonScript {
        python_file_path: std::env::args().nth(1).expect("No python file given"),
        python_env_binary_path: std::env::args().nth(2).expect("No python binary was given"),
    };
    python_script.run()
}
