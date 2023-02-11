use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use rand::seq::SliceRandom;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserAgent {
    firefox: Vec<String>,
    chrome: Vec<String>,
    safari: Vec<String>,
    opera: Vec<String>,
    ie: Vec<String>,
    edge: Vec<String>,
}

impl UserAgent {
    pub fn new() -> Self {
        let file = File::open("user_agents.json").expect("Failed to open file");
        let reader = BufReader::new(file);
        let user_agents: UserAgent = serde_json::from_reader(reader).expect("Failed to parse JSON");
        user_agents
    }

    pub fn random(&self) -> &str {
        let mut rng = rand::thread_rng();
        let field_names = vec!["firefox", "chrome", "safari", "opera", "ie", "edge"];
        let field_name = field_names.choose(&mut rng).unwrap();
        let user_agent = match field_name {
            &"firefox" => self.firefox.choose(&mut rng).unwrap(),
            &"chrome" => self.chrome.choose(&mut rng).unwrap(),
            &"safari" => self.safari.choose(&mut rng).unwrap(),
            &"opera" => self.opera.choose(&mut rng).unwrap(),
            &"ie" => self.ie.choose(&mut rng).unwrap(),
            &"edge" => self.edge.choose(&mut rng).unwrap(),
            _ => panic!("Unknown field name"),
        };
        &user_agent
    }
}
