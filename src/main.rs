use clap::Parser;
use std::time::Duration;
use wayland_client::{Connection, Dispatch, QueueHandle};

/// A powerful and fast autoclicker for Wayland.
#[derive(Parser, Debug)]
#[command(author = "Dacraezy1", version, about, long_about = None)]
struct Args {
    /// Interval between clicks in milliseconds
    #[arg(short, long, default_value_t = 100)]
    interval: u64,
}

struct AppState;

// The Dispatch trait defines how to react to Wayland events.
// This implementation handles the wl_registry events to discover globals.
impl Dispatch<wayland_client::protocol::wl_registry::WlRegistry, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &wayland_client::protocol::wl_registry::WlRegistry,
        event: wayland_client::protocol::wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        // When the compositor advertises a global, print its interface and version.
        // This is crucial for knowing what capabilities are available.
        if let wayland_client::protocol::wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[Registry] Got a global: {} (version {})", interface, version);
        }
    }
}

fn main() {
    let args = Args::parse();

    println!(
        "Starting autoclicker with an interval of {} milliseconds.",
        args.interval
    );

    // Establish a connection to the Wayland display server.
    // The server address is typically read from the WAYLAND_DISPLAY environment variable.
    let conn = match Connection::connect_to_env() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to Wayland display: {}. Is WAYLAND_DISPLAY set?", e);
            return;
        }
    };

    // Create a new event queue for handling Wayland events.
    let mut event_queue = conn.new_event_queue();
    // Get a handle to the event queue, which we can pass to other objects.
    let qhandle = event_queue.handle();

    // Get the main display object.
    let display = conn.display();
    
    // Request the wl_registry object from the compositor.
    // The compositor will respond with a series of global events.
    display.get_registry(&qhandle, ());

    let mut app_state = AppState;

    println!("--- Discovering available Wayland protocols ---");
    // Process the event queue to receive the global events from the registry.
    // This is a blocking call that will wait for the server to respond.
    event_queue.roundtrip(&mut app_state).unwrap();
    println!("---------------------------------------------");


    // --- Main Loop Placeholder ---
    // This is where the click events will be generated in the future.
    let click_interval = Duration::from_millis(args.interval);
    println!("
Starting click loop (placeholder)... Press Ctrl+C to stop.");
    
    loop {
        // In the future, this will dispatch a virtual click event.
        println!("*click*");
        std::thread::sleep(click_interval);
    }
}