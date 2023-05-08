use std::process::Command;

use serde::Deserialize;
use std::fs;
#[derive(Debug, Deserialize)]
struct YamlScript<'a> {
    #[serde(borrow)]
    scripts: Vec<Script<'a>>,
}

#[derive(Debug, Deserialize)]
struct Script<'a> {
    script: &'a str,
    python_exe: &'a str,
    name: &'a str,
}
fn main() -> Result<(), serde_yaml::Error> {
    let contents = fs::read_to_string("dummy_python_scripts/simple_yaml.yml")
        .expect("Should have been able to read the file");
    let yaml_struct: YamlScript = serde_yaml::from_str(&contents)?;

    let output = Command::new(yaml_struct.scripts[0].python_exe)
        .arg(yaml_struct.scripts[0].script)
        .output()
        .expect("failed to execute process");
    println!("Status code is: \n{:#?}", output);
    Ok(())
}
