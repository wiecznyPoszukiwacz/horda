mod action;
mod command;
mod config;
mod horda;
mod hotkey;

use crate::{
    command::Command,
    config::{Config, load_config},
    horda::HordaApp,
    hotkey::start_listener,
};
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_string = load_config()?;
    let config: Config = toml::from_str(&config_string)?;

    let commands: Vec<Command> = config.commands.into_iter().map(Command::from).collect();

    for cmd in &commands {
        println!("--- {}", cmd.name);
        println!("{}", cmd.action.describe());
    }

    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || start_listener(sender));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_visible(true)
            .with_decorations(false)
            .with_always_on_top()
            .with_inner_size([400.0, 60.0])
            .with_position([760.00, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "horda",
        options,
        Box::new(|_cc| Ok(Box::new(HordaApp::new(receiver, commands)))),
    )?;

    Ok(())
}
