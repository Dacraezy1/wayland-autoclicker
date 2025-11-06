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

The autoclicker is controlled via command-line arguments.

```sh
./target/release/wayland-autoclicker --interval <MILLISECONDS>
```

-   `--interval <MILLISECONDS>`: Sets the time in milliseconds between each click.

*(More options will be added in the future.)*

## License

This project is licensed under the **GNU General Public License v3.0**. See the `LICENSE` file for the full text.
