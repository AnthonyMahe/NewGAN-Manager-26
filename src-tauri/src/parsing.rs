
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub uid: String,
    pub name: String,
    pub nationality: String,
    pub ethnicity: String, // Parsing from raw column, might be number or text
    pub mapped_ethnicity: Option<String>,
}

pub fn parse_rtf(path: &str) -> Result<Vec<Player>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut players = Vec::new();

    // Regex to identify lines with UIDs (7+ digits)
    let uid_regex = Regex::new(r"([0-9]){7,}").map_err(|e| e.to_string())?;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if uid_regex.is_match(&line) {
            let parts: Vec<&str> = line.split('|').collect();
            // Expected format based on standard NewGAN view:
            // | UID | Name | Nation | ... | Ethnicity | ...
            // Python: fields[1], fields[2], fields[3], fields[7]
            if parts.len() > 7 {
                let uid = parts[1].trim().to_string();
                let name = parts[2].trim().to_string();
                let nationality = parts[3].trim().to_string();
                let ethnicity = parts[7].trim().to_string();

                players.push(Player {
                    uid,
                    name,
                    nationality,
                    ethnicity, // This is the raw value like "1", "10", "African" etc.
                    mapped_ethnicity: None,
                });
            }
        }
    }

    Ok(players)
}
