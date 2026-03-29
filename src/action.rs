use std::os::unix::process::CommandExt;
use std::time::Duration;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ConnectionExt as XProtoExt, InputFocus, KEY_PRESS_EVENT, KEY_RELEASE_EVENT,
};
use x11rb::protocol::xtest::ConnectionExt as XTestExt;
use x11rb::rust_connection::RustConnection;
pub enum Action {
    Launch(String, Vec<String>),
    Keystrokes(String),
}

impl Action {
    #[allow(dead_code)]
    pub fn describe(&self) -> String {
        match self {
            Action::Launch(program, _) => format!("uruchom {program}"),
            Action::Keystrokes(keystrokes) => format!("wysyła klawisze {keystrokes}"),
        }
    }
}

impl Action {
    pub fn execute(&self, previous_window_id: u32, x11: &x11rb::rust_connection::RustConnection) {
        match self {
            Action::Launch(program, args) => {
                // Detach child from horda's process group so it survives horda restart
                std::process::Command::new(program)
                    .args(args)
                    .process_group(0)
                    .spawn()
                    .ok();
            }
            Action::Keystrokes(keys) => send_keystrokes(keys, x11, previous_window_id),
        };
    }
}

fn send_keystrokes(keys: &str, conn: &RustConnection, window_id: u32) {
    conn.set_input_focus(InputFocus::POINTER_ROOT, window_id, 0u32)
        .expect("focus");
    conn.flush().expect("flush");
    let codes = parse_keys(keys);
    for key in &codes {
        let keysym = key_name_to_keysym(key).expect("err");
        let keycode = keysym_to_keycode(keysym, conn).expect("ha") as u8;

        conn.xtest_fake_input(KEY_PRESS_EVENT, keycode, 0, 0, 0, 0, 0)
            .expect("E4");
    }

    conn.flush().expect("flush");
    std::thread::sleep(Duration::from_millis(100));

    for key in codes.iter().rev() {
        let keysym = key_name_to_keysym(key).expect("err");
        let keycode = keysym_to_keycode(keysym, conn).expect("ha") as u8;

        conn.xtest_fake_input(KEY_RELEASE_EVENT, keycode, 0, 0, 0, 0, 0)
            .expect("E5");
    }
    conn.flush().expect("flush");
}

fn keysym_to_keycode(keysym: u32, conn: &RustConnection) -> Option<u8> {
    let x11_setup = conn.setup();

    let min_keycode = x11_setup.min_keycode;
    let max_keycode = x11_setup.max_keycode;
    let count = max_keycode - min_keycode + 1;

    let keyboard_mapping = conn
        .get_keyboard_mapping(min_keycode, count)
        .expect("can't get keyboard mapping")
        .reply()
        .expect("can't get keyboard mapping");

    let chunks = keyboard_mapping
        .keysyms
        .chunks(keyboard_mapping.keysyms_per_keycode as usize);

    for (i, chunk) in chunks.enumerate() {
        if chunk.contains(&keysym) {
            return Some(min_keycode + i as u8);
        }
    }

    None
}

fn parse_keys(input: &str) -> Vec<&str> {
    input.split('+').collect()
}

fn key_name_to_keysym(name: &str) -> Option<u32> {
    Some(match name.to_lowercase().as_str() {
        "shift" => 0xffe1,
        "ctrl" => 0xffe3,
        "alt" => 0xffe9,
        k => k.chars().next()? as u32,
    })
}
