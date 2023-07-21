use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn create_config_file(path: String) -> Result<(), String> {
    let config = r#"
"Console Sample v.1"
{
 "uri" "http://127.0.0.1:8080/update"
 "timeout" "5.0"
 "buffer"  "0.1"
 "throttle" "0.5"
 "heartbeat" "60.0"
 "auth"
 {
   "token" "CCWJu64ZV3JHDT8hZc"
 }
 "output"
 {
   "precision_time" "3"
   "precision_position" "1"
   "precision_vector" "3"
 }
 "data"
 {
   "provider"            "1"
   "map"                 "1"
   "round"               "1"
   "player_id"           "1"
   "player_state"        "1"
   "player_weapons"      "1"
   "player_match_stats"  "1"
 }
}
"#;

    let path = Path::new(&path).join("gamestate_integration_yourservicenamehere.cfg");
    let mut file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(config.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}
