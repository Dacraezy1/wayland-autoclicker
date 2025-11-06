use clap::Parser;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use wayland_client::{
    protocol::{wl_pointer, wl_registry},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::unstable::virtual_pointer::v1::client::{
    zwp_virtual_pointer_manager_v1::ZwpVirtualPointerManagerV1,
    zwp_virtual_pointer_v1::ZwpVirtualPointerV1,
};
use evdev::{Device, EventType, Key};

/// A powerful and fast autoclicker for Wayland.
#[derive(Parser, Debug)]
#[command(author = "Dacraezy1", version, about, long_about = None)]
struct Args {
    /// Interval between clicks in milliseconds
    #[arg(short, long, default_value_t = 100)]
    interval: u64,

    /// Key to toggle the autoclicker on/off (e.g., F6, X, BTN_LEFT)
    #[arg(short, long, default_value = "F6")]
    toggle_key: String,

    /// Mouse button to click (left, right, middle)
    #[arg(short, long, default_value = "left")]
    button: String,
}

// AppState will hold our Wayland objects and the virtual pointer manager
struct AppState {
    virtual_pointer_manager: Option<ZwpVirtualPointerManagerV1>,
    // We don't need to store the virtual_pointer here, as it's created and used in main.
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            // Bind to the virtual pointer manager if available
            if interface == ZwpVirtualPointerManagerV1::interface().name {
                println!("Found virtual pointer manager (version {})", version);
                state.virtual_pointer_manager = Some(
                    registry.bind::<ZwpVirtualPointerManagerV1, _, _>(name, version, qhandle, ()),
                );
            }
        }
    }
}

// Dispatch for ZwpVirtualPointerManagerV1 (no events to handle for this object)
impl Dispatch<ZwpVirtualPointerManagerV1, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpVirtualPointerManagerV1,
        _event: <ZwpVirtualPointerManagerV1 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

// Dispatch for ZwpVirtualPointerV1 (no events to handle for this object)
impl Dispatch<ZwpVirtualPointerV1, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpVirtualPointerV1,
        _event: <ZwpVirtualPointerV1 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

// Function to parse the toggle key string into an evdev::Key
fn parse_toggle_key(key_str: &str) -> Option<Key> {
    match key_str.to_uppercase().as_str() {
        "F1" => Some(Key::KEY_F1),
        "F2" => Some(Key::KEY_F2),
        "F3" => Some(Key::KEY_F3),
        "F4" => Some(Key::KEY_F4),
        "F5" => Some(Key::KEY_F5),
        "F6" => Some(Key::KEY_F6),
        "F7" => Some(Key::KEY_F7),
        "F8" => Some(Key::KEY_F8),
        "F9" => Some(Key::KEY_F9),
        "F10" => Some(Key::KEY_F10),
        "F11" => Some(Key::KEY_F11),
        "F12" => Some(Key::KEY_F12),
        "X" => Some(Key::KEY_X),
        "Z" => Some(Key::KEY_Z),
        "C" => Some(Key::KEY_C),
        "V" => Some(Key::KEY_V),
        "B" => Some(Key::KEY_B),
        "N" => Some(Key::KEY_N),
        "M" => Some(Key::KEY_M),
        "A" => Some(Key::KEY_A),
        "S" => Some(Key::KEY_S),
        "D" => Some(Key::KEY_D),
        "W" => Some(Key::KEY_W),
        "Q" => Some(Key::KEY_Q),
        "E" => Some(Key::KEY_E),
        "R" => Some(Key::KEY_R),
        "T" => Some(Key::KEY_T),
        "Y" => Some(Key::KEY_Y),
        "U" => Some(Key::KEY_U),
        "I" => Some(Key::KEY_I),
        "O" => Some(Key::KEY_O),
        "P" => Some(Key::KEY_P),
        "K" => Some(Key::KEY_K),
        "L" => Some(Key::KEY_L),
        "J" => Some(Key::KEY_J),
        "H" => Some(Key::KEY_H),
        "G" => Some(Key::KEY_G),
        "F" => Some(Key::KEY_F),
        "BTN_LEFT" => Some(Key::BTN_LEFT),
        "BTN_RIGHT" => Some(Key::BTN_RIGHT),
        "BTN_MIDDLE" => Some(Key::BTN_MIDDLE),
        _ => None,
    }
}

// Function to parse the mouse button string into a wl_pointer::Button
fn parse_mouse_button(button_str: &str) -> Option<u32> {
    match button_str.to_lowercase().as_str() {
        "left" => Some(wl_pointer::Button::Left as u32),
        "right" => Some(wl_pointer::Button::Right as u32),
        "middle" => Some(wl_pointer::Button::Middle as u32),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let toggle_key = parse_toggle_key(&args.toggle_key)
        .ok_or_else(|| format!("Invalid toggle key: {}", args.toggle_key))?;

    let mouse_button = parse_mouse_button(&args.button)
        .ok_or_else(|| format!("Invalid mouse button: {}. Use 'left', 'right', or 'middle'.", args.button))?;

    println!(
        "Autoclicker configured: Interval = {}ms, Toggle Key = {}, Mouse Button = {}",
        args.interval, args.toggle_key, args.button
    );
    println!("To start/stop clicking, press the '{}' key.", args.toggle_key);
    println!("NOTE: This program needs to be run with permissions to read input devices (e.g., `sudo`).");

    // Shared state for toggling the autoclicker
    let clicking_enabled = Arc::new(Mutex::new(false));
    let clicking_enabled_clone = Arc::clone(&clicking_enabled);

    // --- Keyboard Listener Thread ---
    thread::spawn(move || {
        let mut device = None;
        // Try to find a keyboard device
        for d in evdev::enumerate() {
            if d.supported_events().contains(EventType::KEY) {
                println!("Found input device: {} ({:?})", d.name().unwrap_or("unnamed"), d.physical_path().unwrap_or("unknown"));
                // Heuristic: try to find a keyboard. This might need refinement.
                // A more robust solution would be to let the user specify the device path.
                if d.name().unwrap_or("").to_lowercase().contains("keyboard") || d.physical_path().unwrap_or("").to_lowercase().contains("kbd") {
                    device = Some(d);
                    break;
                }
            }
        }

        let mut device = device.expect("No keyboard device found. Ensure you have permissions to read /dev/input/event*.");
        
        // Grab the device to prevent events from going to other applications
        // This requires root privileges.
        if let Err(e) = device.grab() {
            eprintln!("Failed to grab input device: {}. Ensure you run with sufficient permissions (e.g., `sudo`).", e);
            return;
        }
        println!("Grabbed input device: {}", device.name().unwrap_or("unnamed"));

        loop {
            for event in device.fetch_events().unwrap() {
                if let evdev::InputEventKind::Key(key_event) = event.kind() {
                    if key_event.state() == evdev::KeyState::Pressed && key_event.key() == toggle_key {
                        let mut enabled = clicking_enabled_clone.lock().unwrap();
                        *enabled = !*enabled; // Toggle the state
                        println!("Autoclicker toggled: {}", if *enabled { "ON" } else { "OFF" });
                    }
                }
            }
        }
    });

    // --- Wayland Connection and Clicking Logic (Main Thread) ---
    let conn = Connection::connect_to_env()?;
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    let mut app_state = AppState {
        virtual_pointer_manager: None,
    };

    // Process events to get the virtual pointer manager
    event_queue.roundtrip(&mut app_state)?;

    let virtual_pointer_manager = app_state
        .virtual_pointer_manager
        .expect("Compositor does not support zwp_virtual_pointer_manager_v1. Cannot create virtual pointer.");

    // Create the virtual pointer
    let virtual_pointer = virtual_pointer_manager.create_virtual_pointer(&qhandle, ());

    println!("Virtual pointer created. Autoclicker ready.");

    let click_interval = Duration::from_millis(args.interval);

    loop {
        let enabled = *clicking_enabled.lock().unwrap();

        if enabled {
            // Send button press
            virtual_pointer.button(conn.display().get_last_serial(), 0, mouse_button, wl_pointer::ButtonState::Pressed);
            virtual_pointer.frame(); // Commit the event
            conn.flush()?;

            thread::sleep(Duration::from_millis(10)); // Small delay for button down state

            // Send button release
            virtual_pointer.button(conn.display().get_last_serial(), 0, mouse_button, wl_pointer::ButtonState::Released);
            virtual_pointer.frame(); // Commit the event
            conn.flush()?;

            thread::sleep(click_interval.checked_sub(Duration::from_millis(10)).unwrap_or_default());
        } else {
            // If not clicking, sleep for a short duration to avoid busy-waiting
            thread::sleep(Duration::from_millis(50));
        }
    }
}
