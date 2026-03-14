pub enum Action {
    Launch(String),
    Keystrokes(String),
}

impl Action {
    pub fn describe(&self) -> String {
        match self {
            Action::Launch(program) => format!("uruchom {program}"),
            Action::Keystrokes(keystrokes) => format!("wysyła klawisze {keystrokes}"),
        }
    }
}

impl Action {
    pub fn execute(&self) {
        match self {
            Action::Launch(program) => std::process::Command::new(program).spawn().ok(),
            Action::Keystrokes(_keys) => todo!("dodaj kistroki"),
        };
    }
}
