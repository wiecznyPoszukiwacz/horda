use serde::Deserialize;
pub fn load_config(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[derive(Deserialize)]
pub struct Config {
    pub(crate) commands: Vec<CommandConfig>,
}

#[derive(Deserialize)]
pub struct CommandConfig {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) action_type: String,
    pub(crate) action_value: String,
}
