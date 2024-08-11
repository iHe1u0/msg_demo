use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn get_cache_path() -> String {
    let mut dir = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Executable has no parent directory")
        .to_path_buf();

    dir.push("data.json");

    dir.to_str()
        .expect("Failed to convert path to string")
        .to_string()
}

pub fn save_data(path: &str, data: Vec<&str>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    for s in data {
        file.write_all(s.as_bytes())?;
        // write new line
        file.write_all(b"\n")?;
    }

    Ok(())
}

pub fn read_data(path: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let content = String::from_utf8_lossy(&buffer);
    let strings: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    Ok(strings)
}
