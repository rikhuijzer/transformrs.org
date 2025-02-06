use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

fn code_blocks(content: &str, lang: &str) -> Vec<String> {
    let re = Regex::new(&format!(r"```{lang}\n([\s\S]*?)```")).unwrap();

    re.captures_iter(&content)
        .filter_map(|cap| cap.get(1).map(|code| code.as_str().to_string()))
        .collect()
}

fn create_project(dir: &Path, dependencies: &str, code: &str) {
    let project = format!(
        r#"
        [package]
        name = "transformrs-org-test"
        version = "0.1.0"
        edition = "2021"

        {dependencies}
    "#
    );
    fs::write(dir.join("Cargo.toml"), project).unwrap();

    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.rs"), code).unwrap();
}

fn copy_env(dir: &Path) {
    let env = fs::read_to_string(".env").unwrap();
    fs::write(dir.join(".env"), env).unwrap();
}

fn run_project(dir: &Path) {
    println!("Building project...");
    let _output = Command::new("cargo")
        .arg("build")
        .current_dir(dir)
        .output()
        .expect("Failed to run cargo build");

    println!("Running project...");
    let output = Command::new("cargo")
        .arg("run")
        .current_dir(dir)
        .output()
        .expect("Failed to run cargo run");

    println!("Output: {:?}", String::from_utf8(output.stdout).unwrap());
}

fn main() {
    let content =
        fs::read_to_string("content/_index.md").expect("Couldn't read _index.md");

    let dependencies = code_blocks(&content, "toml").first().unwrap().clone();
    let code_blocks = code_blocks(&content, "rust");

    for code_block in code_blocks {
        println!("\nBuilding and running code block:\n{code_block}\n");
        let tmp_dir = tempfile::tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path();
        create_project(tmp_dir_path, &dependencies, &code_block);
        copy_env(tmp_dir_path);
        run_project(tmp_dir_path);
        drop(tmp_dir);
    }
}
