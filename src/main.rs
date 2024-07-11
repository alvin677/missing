use serde_json;
use colored::Colorize;

fn main() {
    let repository = "https://raw.githubusercontent.com/alvin677/missing/main/repo.json";
    let body = reqwest::blocking::get(repository).unwrap().text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON parse error.");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("{}", "Available packages:".green());
        for p in json.as_object().unwrap().keys() {
            println!("  {} - {}", p.cyan(), json[p]["description"].as_str().unwrap());
        }
        return;
    }

    let package: &str = &args[1];

    if json[package].is_null() {
        eprintln!("{}: package not found", "Error".red());
    }
    else {
        println!("{}: {}..", "Installing".cyan(), json[package]["name"].as_str().unwrap());

        let package_name = json[package]["name"].as_str().unwrap();
        let filename = std::path::Path::new(json[package]["source"].as_str().unwrap());
        let file_ext = filename.extension().unwrap().to_str().unwrap();
        let file_source = json[package]["source"].as_str().unwrap();

        std::process::Command::new("cmd")
            .arg(format!("/C curl -o {}-installer.{} {}", package_name, file_ext, file_source))
            .output()
            .expect("Failed to execute command, is cURL installed?");
            
        println!("{}: executing installer", "Downloaded".yellow());

        std::process::Command::new("cmd")
            .arg(format!("/C start {}-installer.{}", package_name, file_ext))
            .output()
            .expect("Failed to open installer.");

        //std::fs::remove_file(format!("{}-installer.{}", package_name, file_ext)).unwrap();
        println!("{}: follow installer instructions", "Finished".green());
    }
}
