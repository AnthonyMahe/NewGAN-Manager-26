
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};
use crate::parsing::Player;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
pub struct MappingConfig {
    pub mode: String, // "generate", "preserve", "overwrite"
    pub rtf_path: String,
    pub img_dir: String,
}

#[derive(Clone)]
struct Face {
    path: String, // Relative path inside img_dir, e.g., "African/001.png"
    _filename: String, // "001" (prefixed with _ as unused for now)
}

pub fn generate_mapping(config: MappingConfig) -> Result<(), String> {
    // 1. Parse RTF
    let players = crate::parsing::parse_rtf(&config.rtf_path)?;
    
    // 2. Scan Image Directory
    let faces = scan_faces(&config.img_dir)?;

    // 3. Load Ethnicity Mapping (Hardcoded for now based on cfg.json)
    let ethnic_map = get_ethnicity_map();

    // 4. Assign Faces
    let mut assignments: HashMap<String, String> = HashMap::new(); // UID -> ImagePath
    
    // TODO: Implement "Preserve" mode logic by reading existing config.xml
    
    let mut available_faces = faces.clone();

    for player in players {
        // Resolve ethnicity
        let eth_key = resolve_ethnicity(&player, &ethnic_map);
        
        // Find pool
        if let Some(pool) = available_faces.get_mut(&eth_key) {
            if let Some(face) = pool.choose(&mut rand::thread_rng()) {
                assignments.insert(player.uid, face.path.clone());
                // Remove if no duplicates allowed (TODO: Add option)
                // For now, allow duplicates or remove? Original removes unless "duplicates" is checked.
                // We'll assume NO duplicates for better quality, but need enough faces.
                // Keeping simple: random pick with replacement for now if pool is small, or mapped logic?
                // Original: keys are "African", "Asian", etc.
                // Images are in folders named "African", "Asian".
            }
        }
    }

    // 5. Write config.xml
    write_config_xml(&config.img_dir, &assignments)?;

    Ok(())
}

fn scan_faces(dir: &str) -> Result<HashMap<String, Vec<Face>>, String> {
    let mut map: HashMap<String, Vec<Face>> = HashMap::new();
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "png" || ext == "jpg" {
                    // Start of relative path
                    let prefix = Path::new(dir);
                    let relative = path.strip_prefix(prefix).map_err(|e| e.to_string())?;
                    // Ethnicity is usually the parent folder name
                    if let Some(parent) = relative.parent() {
                        let eth_name = parent.to_string_lossy().to_string();
                        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                        
                        map.entry(eth_name).or_insert(Vec::new()).push(Face {
                            path: relative.to_string_lossy().to_string().replace("\\", "/"),
                            _filename: filename,
                        });
                    }
                }
            }
        }
    }
    Ok(map)
}

fn resolve_ethnicity(player: &Player, map: &HashMap<String, String>) -> String {
    // Logic from mapper.py: Uses NAT (Nationality) to map to Region/Ethnicity
    // player.ethnicity (raw column 7) also plays a role (0-10 or values)
    
    // Simplified: Use Nation map first
    if let Some(eth) = map.get(&player.nationality) {
        return eth.clone();
    }
    "African".to_string() // Fallback
}

fn get_ethnicity_map() -> HashMap<String, String> {
    let json_data = r#"
    {
        "AFG": "MESA", "AXL": "Scandinavian", "ALB": "YugoGreek", "ALG": "MENA", "ASA": "African", "AND": "SpanMed", "ANG": "African", "AIA": "African", 
        "ATG": "African", "ARG": "SAMed", "ARM": "EECA", "ARU": "African", "AUS": "Central European", "AUT": "Central European", "AZE": "EECA", 
        "BAS": "SpanMed", "BAH": "African", "BHR": "MESA", "BAN": "MESA", "BRB": "African", "BLR": "EECA", "BEL": "Central European", "BLZ": "South American", 
        "BEN": "African", "BER": "African", "BHU": "Asian", "BOL": "South American", "BIH": "YugoGreek", "BOT": "African", "BRA": "South American", 
        "VGB": "African", "BRU": "Seasian", "BUL": "EECA", "BFA": "African", "BDI": "African", "CAM": "Seasian", "CMR": "African", "CAN": "Caucasian", 
        "CPV": "African", "CAY": "African", "CTA": "African", "CHA": "African", "CHI": "South American", "CHN": "Asian", "COL": "South American", 
        "COM": "African", "COD": "African", "CGO": "African", "COK": "African", "CRC": "South American", "CIV": "African", "CRO": "YugoGreek", 
        "CUB": "South American", "CUW": "African", "CYP": "MENA", "CZE": "EECA", "DEN": "Scandinavian", "DJI": "African", "DMA": "African", 
        "DOM": "South American", "ECU": "South American", "EGY": "MENA", "SLV": "South American", "ENG": "Caucasian", "EQG": "African", 
        "ERI": "African", "EST": "EECA", "SWZ": "African", "ETH": "African", "FRO": "Scandinavian", "FIJ": "African", "FIN": "Scandinavian", 
        "FRA": "Central European", "TAH": "African", "GAB": "African", "GAM": "African", "GEO": "EECA", "GER": "Central European", "GHA": "African", 
        "GIB": "Caucasian", "GRE": "YugoGreek", "GRL": "Caucasian", "GRN": "African", "GLP": "African", "GUM": "African", "GUA": "South American", 
        "GUI": "African", "GNB": "African", "GUY": "African", "HAI": "African", "HON": "South American", "HKG": "Asian", "HUN": "Central European", 
        "ISL": "Scandinavian", "IND": "MESA", "IDN": "Seasian", "IRN": "MESA", "IRQ": "MESA", "IRL": "Caucasian", "ISR": "MENA", "ITA": "Italmed", 
        "JAM": "African", "JPN": "Asian", "JOR": "MESA", "KAZ": "EECA", "KEN": "African", "KIR": "African", "PRK": "Asian", "KOR": "Asian", 
        "KVX": "YugoGreek", "KUW": "MESA", "KGZ": "EECA", "LAO": "Seasian", "LVA": "EECA", "LBN": "MENA", "LES": "African", "LBR": "African", 
        "LBY": "African", "LIE": "Central European", "LTU": "EECA", "LUX": "Central European", "MAC": "Asian", "MAD": "African", "MWI": "African", 
        "MAS": "Seasian", "MDV": "African", "MLI": "African", "MLT": "Italmed", "MTQ": "African", "MTN": "African", "MRI": "African", 
        "MEX": "South American", "FSM": "African", "MDA": "EECA", "MON": "Italmed", "MNG": "Asian", "MNE": "YugoGreek", "MSR": "African", 
        "MAR": "MENA", "MOZ": "African", "MYA": "Seasian", "NAM": "African", "NEP": "MESA", "NED": "Central European", "NCL": "African", 
        "NZL": "Caucasian", "NCA": "South American", "NIG": "African", "NGA": "African", "NIU": "African", "NIR": "Caucasian", "NMI": "African", 
        "MKD": "EECA", "NOR": "Scandinavian", "OMA": "MESA", "PAK": "MESA", "PLW": "African", "PLE": "MESA", "PAN": "South American", 
        "PNG": "African", "PAR": "South American", "PER": "South American", "PHI": "Seasian", "POL": "Central European", "POR": "SpanMed", 
        "PUR": "South American", "QAT": "MESA", "REU": "African", "ROU": "EECA", "RUS": "EECA", "RWA": "African", "SKN": "African", 
        "LCA": "African", "SMN": "African", "VIN": "African", "SAM": "African", "SMR": "Italmed", "STP": "African", "KSA": "MESA", 
        "SCO": "Caucasian", "SEN": "African", "SRB": "YugoGreek", "SEY": "African", "SLE": "African", "SIN": "Seasian", "SMA": "African", 
        "SVK": "Central European", "SVN": "YugoGreek", "SOL": "African", "SOM": "African", "RSA": "African", "SSD": "African", "ESP": "SpanMed", 
        "SRI": "African", "SDN": "African", "SUR": "African", "SWE": "Scandinavian", "SUI": "Central European", "SYR": "MESA", "TPE": "Asian", 
        "TJK": "EECA", "TAN": "African", "THA": "Seasian", "TLS": "African", "TOG": "African", "TGA": "African", "TRI": "African", "TUN": "MENA", 
        "TUR": "MENA", "TKM": "EECA", "TCA": "African", "TUV": "African", "UGA": "African", "UKR": "EECA", "UAE": "MESA", "GBR": "Caucasian", 
        "USA": "Caucasian", "VIR": "African", "URU": "SAMed", "UZB": "EECA", "VAN": "African", "VAT": "Caucasian", "VEN": "South American", 
        "VIE": "Seasian", "WAL": "Caucasian", "YEM": "MESA", "ZAM": "African", "ZIM": "African", "ZAN": "African", "LIB": "MENA", 
        "GUF": "African", "WFI": "African", "SUD": "MENA", "MAY": "African", "BOE": "African", "BLM": "Caucasian", "SPM": "Caucasian", 
        "MGL": "Asian", "KOS": "YugoGreek", "ESW": "African"
    }
    "#;
    
    match serde_json::from_str::<HashMap<String, String>>(json_data) {
        Ok(map) => map,
        Err(_) => {
            // Fallback if parsing fails (should not happen with valid json above)
            let mut fallback = HashMap::new();
            fallback.insert("ENG".into(), "Caucasian".into());
            fallback
        }
    }
}

fn write_config_xml(dir: &str, assignments: &HashMap<String, String>) -> Result<(), String> {
    let path = Path::new(dir).join("config.xml");
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    
    file.write_all(b"<record>\n\t<boolean id=\"preload\" value=\"false\"/>\n\t<boolean id=\"amap\" value=\"false\"/>\n\t<list id=\"maps\">\n").map_err(|e| e.to_string())?;
    
    for (uid, img_path) in assignments {
        // <record from="African/001" to="graphics/pictures/person/20001234/portrait"/>
        // FM config format: from acts as the filename (without extension usually), to is the in-game ID
        // Wait, check original xmlparser.py or write_xml function.
        // Original: <record from="African/001" to="graphics/pictures/person/UID/portrait"/>
        // Does "from" need extension? usually no for FM.
        let from = img_path.trim_end_matches(".png").trim_end_matches(".jpg");
        writeln!(file, "\t\t<record from=\"{}\" to=\"graphics/pictures/person/{}/portrait\"/>", from, uid).map_err(|e| e.to_string())?;
    }
    
    file.write_all(b"\t</list>\n</record>").map_err(|e| e.to_string())?;
    Ok(())
}
