# WhatsappLinuxRust

A lightweight desktop wrapper for [WhatsApp Web](https://web.whatsapp.com), written in Rust. It opens WhatsApp Web in its own standalone window using [`wry`](https://github.com/tauri-apps/wry) and [`tao`](https://github.com/tauri-apps/tao) (WebKitGTK on Linux), so you can use WhatsApp on Linux without keeping a browser tab open.

> **Disclaimer:** This is an unofficial project and is not affiliated with, endorsed by, or connected to WhatsApp or Meta. It simply loads the official WhatsApp Web site in a window. WhatsApp is a trademark of its respective owner.

## Features

- Standalone window, no browser tabs or address bar
- Login is saved between sessions (scan the QR code only once)
- **Save dialog for downloads:** choose where each file goes instead of it landing silently in `~/Downloads` (the last folder you used is remembered)
- **Native desktop notifications** for incoming messages and finished downloads
- **Video playback** with MediaSource and encrypted media enabled
- **Microphone and camera access** (voice notes work); WebRTC is enabled where your WebKitGTK supports it, but calls are not available on most distros (see [Calls](#calls))
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

- Runtime packages for video, media, and notifications:

```bash
sudo apt install gstreamer1.0-plugins-good gstreamer1.0-plugins-bad \
  gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-pipewire \
  libnice10 libnotify-bin
```

## Build and run

```bash
git clone https://github.com/DEAMJAVA/WhatsappLinuxRust
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

## Downloads

When you download a file (or save media from a chat), a native "Save as" dialog opens with the suggested filename. Pick a location and confirm, or cancel to abort the download. The app remembers the last folder you saved to for the rest of the session, and shows a notification when the download completes or fails.

## Notifications

Incoming messages are shown as native desktop notifications via your system's notification daemon (through `libnotify`). Notification permission is granted automatically, so you don't need to click "Allow" in WhatsApp Web.

## Calls

**Voice and video calls do not work on most Linux systems.** WhatsApp Web shows "your browser doesn't support calls" because most distros build WebKitGTK without WebRTC, so the `RTCPeerConnection` API doesn't exist in the engine. No setting or user agent can add it.

The app still enables WebRTC and media-stream settings and auto-grants microphone and camera permission, so if you run a WebKitGTK build that includes WebRTC it will be used. Even then, calls may fail: WebKitGTK does not implement `RTCRtpScriptTransform`, which end-to-end encrypted calls appear to need, and WhatsApp decides server-side whether to offer calls to a given browser.

To check what your system supports, run with debugging on and look for the `[wa-diag]` line in the terminal:

```bash
WHATSAPP_DEBUG=1 ./whatsapp-linux
```

If it shows `RTCPeerConnection=undefined`, your WebKitGTK has no WebRTC. Use the phone app for calls, or a Chromium-based client.

## Environment variables

| Variable | Effect |
|----------|--------|
| `WHATSAPP_DEBUG=1` | Print the web page's console output and a WebRTC capability check to the terminal |
| `WHATSAPP_UA="..."` | Override the user agent (useful if WhatsApp shows "browser not supported") |

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
- **Videos won't play or are silent:** Install the GStreamer packages listed under [Requirements](#requirements).
- **No notifications:** Check that a notification daemon is running (GNOME, KDE, and most desktops include one) and that `libnotify-bin` is installed. Test with `notify-send "test"`.
- **Calls say the browser is unsupported:** Expected on most distros, see [Calls](#calls).
- **User agent keeps showing Safari:** WebKitGTK's site-specific quirks can override it. This app disables them, so make sure you are running the latest `main.rs`.
- **Build fails on a `webkit2gtk` setting:** Setting names can vary between crate versions. Check the compiler error and the `webkit2gtk` crate docs for the matching method.

## Limitations

- No tray icon, and clicking a notification does not focus the window
- No voice/video calls on most systems (WebKitGTK usually lacks WebRTC, see [Calls](#calls))
- External links are not routed to your default browser
- Not a fully static binary, it depends on the system WebKitGTK and GTK libraries

## License

Choose a license for your project (for example, [MIT](https://choosealicense.com/licenses/mit/)) and add a `LICENSE` file, then update this section.
