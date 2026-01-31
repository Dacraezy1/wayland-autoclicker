# Wayclicker

A powerful, universal autoclicker for Linux. 

**Works on ALL desktop environments:**
*   GNOME (Wayland & X11)
*   KDE Plasma (Wayland & X11)
*   Hyprland
*   Sway
*   XFCE, Mate, etc.

Created by: **Dacraezy1**

## 🚀 How it Works
Unlike other Wayland autoclickers that rely on compositor-specific protocols (like `wlr-virtual-pointer`), **Wayclicker** uses the Linux kernel's `uinput` subsystem to create a virtual mouse device. This allows it to work universally across any Linux system, regardless of the display server (Wayland/X11) or compositor.

## 🛠️ Installation

### Option 1: Download Binary (Recommended)
1.  Go to the [Releases](https://github.com/Dacraezy1/wayclicker/releases) page (or check the Actions tab if you just forked this).
2.  Download the `wayclicker` binary.
3.  Make it executable:
    ```bash
    chmod +x wayclicker
    ```
4.  Run it:
    ```bash
    sudo ./wayclicker
    ```

### Option 2: Build from Source
**Prerequisites:** Rust toolchain.

1.  Clone this repository.
2.  Build:
    ```bash
    cargo build --release
    ```
3.  Run:
    ```bash
    sudo ./target/release/wayclicker
    ```

## 🎮 Usage

Because `Wayclicker` operates at the kernel level (`/dev/uinput` and `/dev/input/*`), **root privileges (sudo) are required**.

```bash
sudo ./wayclicker [OPTIONS]
```

### Options

*   `-i, --interval <MS>`: Time in milliseconds between clicks (Default: `100`).
*   `-t, --toggle-key <KEY>`: The key to toggle clicking on/off (Default: `F6`).
    *   Supported: `F1`-`F12`, `A`-`Z`, `BTN_LEFT`, `BTN_RIGHT`, `SHIFT`, `CTRL`, `ALT`, etc.
*   `-b, --button <BTN>`: The mouse button to click (Default: `left`).
    *   Options: `left`, `right`, `middle`.

### Examples

**1. Basic usage (Click every 100ms, toggle with F6):**
```bash
sudo ./wayclicker
```

**2. Fast clicking (25ms) using the 'X' key to toggle:**
```bash
sudo ./wayclicker --interval 25 --toggle-key X
```

**3. Spam Right-Click with 500ms interval:**
```bash
sudo ./wayclicker --button right --interval 500
```

## ⚠️ Important Note
This tool grabs your keyboard input device to listen for the toggle key. While running, some keys might be intercepted exclusively by the autoclicker depending on the "grab" implementation. To exit the program, you can usually press `Ctrl+C` in the terminal where it is running.

## License
This project is licensed under the **GNU General Public License v3.0**. See the `LICENSE` file for the full text.