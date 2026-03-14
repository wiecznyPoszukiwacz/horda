use rdev::{Event, EventType, Key, listen};
use std::sync::atomic::Ordering;
use std::sync::mpsc;

pub(crate) fn start_listener(sender: mpsc::Sender<String>) {
    static IS_ALT_PRESSED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    let _ = listen(move |event: Event| {
        // nie zwijaj mi tego kurwo

        match event.event_type {
            EventType::KeyPress(Key::Alt) => IS_ALT_PRESSED.store(true, Ordering::SeqCst),
            EventType::KeyRelease(Key::Alt) => IS_ALT_PRESSED.store(false, Ordering::SeqCst),
            EventType::KeyPress(Key::Space) => {
                if IS_ALT_PRESSED.load(Ordering::SeqCst) {
                    sender.send("Alt+Space".to_string()).ok();
                }
            }
            _ => {}
        };
    });
}
