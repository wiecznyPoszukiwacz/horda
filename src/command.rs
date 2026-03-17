use crate::{action::Action, config::CommandConfig};
pub struct Command {
    pub name: String,
    pub description: String,
    pub(crate) action: Action,
}
impl From<CommandConfig> for Command {
    fn from(cfg: CommandConfig) -> Self {
        let action = match cfg.action_type.as_str() {
            "launch" => Action::Launch(cfg.action_value),
            "keystrokes" => Action::Keystrokes(cfg.action_value),
            _ => panic!("nieznany typ"),
        };

        Command {
            name: cfg.name,
            description: cfg.description,
            action,
        }
    }
}
