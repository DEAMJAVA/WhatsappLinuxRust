# WhatsappLinuxRust

A minimal, lightweight desktop wrapper for [WhatsApp Web](https://web.whatsapp.com), written in Rust. It opens WhatsApp Web in its own standalone window using [`wry`](https://github.com/tauri-apps/wry) and [`tao`](https://github.com/tauri-apps/tao) (WebKitGTK on Linux), so you can use WhatsApp on Linux without keeping a browser tab open.

> **Disclaimer:** This is an unofficial project and is not affiliated with, endorsed by, or connected to WhatsApp or Meta. It simply loads the official WhatsApp Web site in a window. WhatsApp is a trademark of its respective owner.

## Features

- Standalone window, no browser tabs or address bar
- Login is saved between sessions (scan the QR code only once)
- Small codebase (a single `main.rs`)
- No bundled browser engine, uses the system WebKitGTK

## Requirements

- Linux (developed and tested on Ubuntu)
- Rust toolchain ([install via rustup](https://rustup.rs))
- System libraries:

```bash
sudo apt install build-essential libwebkit2gtk-4.1-dev libgtk-3-dev \
  libsoup-3.0-dev libjavascriptcoregtk-4.1-dev pkg-config
```

## Build and run

```bash
git clone https://github.com/DEAMJAVA/whatsapp-linux.git
cd whatsapp-linux
cargo run --release
```

On first launch, open WhatsApp on your phone, go to **Linked Devices**, and scan the QR code.

## Install

Build the release binary and copy it somewhere on your `PATH`:

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/whatsapp-linux ~/.local/bin/
```

### Add an app menu entry (optional)

Create `~/.local/share/applications/whatsapp-linux.desktop`:

```ini
[Desktop Entry]
Name=WhatsApp
Exec=/home/YOUR_USER/.local/bin/whatsapp-linux
Icon=/home/YOUR_USER/.local/share/icons/whatsapp.png
Type=Application
Categories=Network;InstantMessaging;
StartupWMClass=whatsapp-linux
```

Replace `YOUR_USER` with your username, add any PNG icon at the path above, then run:

```bash
update-desktop-database ~/.local/share/applications
```

## Data location

Cookies and local storage are saved in:

```
~/.local/share/whatsapp-linux
```

Delete this folder to log out and reset the app.

## Troubleshooting

- **"Browser not supported" page:** WhatsApp Web checks the browser version. Update the Chrome version number in the `USER_AGENT` string in `src/main.rs` and rebuild.
- **Build fails with missing library errors:** Make sure all the system packages listed under [Requirements](#requirements) are installed.
- **Window opens blank:** Check that `libwebkit2gtk-4.1-dev` is installed, and try running from a terminal to see any error output.

## Limitations

- No tray icon or native desktop notifications
- Voice and video calls are not available (same as WhatsApp Web)
- Not a fully static binary, it depends on the system WebKitGTK and GTK libraries

## License

Choose a license for your project (for example, [MIT](https://choosealicense.com/licenses/mit/)) and add a `LICENSE` file, then update this section.
