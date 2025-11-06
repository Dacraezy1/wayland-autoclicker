# Wayland Autoclicker

A powerful and fast autoclicker designed for Linux systems using the Wayland display server.

Created by: **Dacraezy1**

## Building

This project is built with Rust. You will need the Rust toolchain installed.

1.  Clone this repository.
2.  Navigate to the project directory:
    ```sh
    cd wayland-autoclicker
    ```
3.  Build the project in release mode:
    ```sh
    cargo build --release
    ```
4.  The executable will be located at `target/release/wayland-autoclicker`.

## Usage

The autoclicker is controlled via command-line arguments and a toggle key.

**Important:** This program needs to be run with permissions to read input devices and create virtual ones. You will likely need to run it with `sudo`.

```sh
sudo ./target/release/wayland-autoclicker [OPTIONS]
```

### Options:

-   `--interval <MILLISECONDS>`: Sets the time in milliseconds between each click. (Default: `100`)
-   `--toggle-key <KEY>`: Specifies the keyboard key to toggle the autoclicker on/off.
    *   Examples: `F6`, `X`, `BTN_LEFT` (for a mouse button).
    *   (Default: `F6`)
-   `--button <BUTTON>`: Specifies which mouse button to click.
    *   Options: `left`, `right`, `middle`.
    *   (Default: `left`)

### Example:

To run the autoclicker with a 50ms interval, toggled by the `F8` key, clicking the right mouse button:

```sh
sudo ./target/release/wayland-autoclicker --interval 50 --toggle-key F8 --button right
```

## License

This project is licensed under the **GNU General Public License v3.0**. See the `LICENSE` file for the full text.