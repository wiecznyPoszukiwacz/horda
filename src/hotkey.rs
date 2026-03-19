use rdev::{Event, EventType, Key, grab};
use std::cell::Cell;
use std::sync::mpsc;

/// Starts a global keyboard listener that detects the Alt+Space shortcut.
/// The shortcut is ignored when Ctrl, Shift or Meta is simultaneously held.
/// Alt+Space is consumed so other applications do not receive it.
pub(crate) fn start_listener(sender: mpsc::Sender<String>) {
    let lalt_pressed  = Cell::new(false);
    let ralt_pressed  = Cell::new(false);
    let lctrl_pressed = Cell::new(false);
    let rctrl_pressed = Cell::new(false);
    let lshift_pressed = Cell::new(false);
    let rshift_pressed = Cell::new(false);
    let lmeta_pressed  = Cell::new(false);
    let rmeta_pressed  = Cell::new(false);

    let _ = grab(move |event: Event| {
        // nie zwijaj mi tego kurwo
        match event.event_type {
            EventType::KeyPress(Key::Alt)     => { lalt_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::Alt)   => { lalt_pressed.set(false); Some(event) }
            EventType::KeyPress(Key::AltGr)   => { ralt_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::AltGr) => { ralt_pressed.set(false); Some(event) }

            EventType::KeyPress(Key::ControlLeft)    => { lctrl_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::ControlLeft)  => { lctrl_pressed.set(false); Some(event) }
            EventType::KeyPress(Key::ControlRight)   => { rctrl_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::ControlRight) => { rctrl_pressed.set(false); Some(event) }

            EventType::KeyPress(Key::ShiftLeft)    => { lshift_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::ShiftLeft)  => { lshift_pressed.set(false); Some(event) }
            EventType::KeyPress(Key::ShiftRight)   => { rshift_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::ShiftRight) => { rshift_pressed.set(false); Some(event) }

            EventType::KeyPress(Key::MetaLeft)    => { lmeta_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::MetaLeft)  => { lmeta_pressed.set(false); Some(event) }
            EventType::KeyPress(Key::MetaRight)   => { rmeta_pressed.set(true);  Some(event) }
            EventType::KeyRelease(Key::MetaRight) => { rmeta_pressed.set(false); Some(event) }

            EventType::KeyPress(Key::Space) => {
                let alt   = lalt_pressed.get() || ralt_pressed.get();
                let ctrl  = lctrl_pressed.get() || rctrl_pressed.get();
                let shift = lshift_pressed.get() || rshift_pressed.get();
                let meta  = lmeta_pressed.get() || rmeta_pressed.get();

                if alt && !ctrl && !shift && !meta {
                    sender.send("Alt+Space".to_string()).ok();
                    None
                } else {
                    Some(event)
                }
            }
            _ => Some(event)
        }
    });
}
