use serde::Deserialize;
pub fn load_config() -> Result<(String, String), std::io::Error> {
    let homedir = std::env::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "home dir not found",
    ))?;

    let homedir = homedir.to_str().ok_or(std::io::Error::other(
        "invalid UTF-8 in home directory path",
    ))?;

    let config_path = format!("{homedir}/.config/horda/horda.toml");
    let content = std::fs::read_to_string(&config_path)?;

    Ok((content, config_path))
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
    #[serde(default)]
    pub(crate) action_args: Vec<String>,
}
