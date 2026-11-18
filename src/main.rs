use std::{cell::{Cell, RefCell}, path::PathBuf, rc::Rc};

use gtk::prelude::*;
use notify_rust::Notification;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowExtUnix,
    window::WindowBuilder,
};
use webkit2gtk::{
    DownloadExt, NotificationExt, NotificationPermissionRequest, PermissionRequestExt,
    SettingsExt, UserMediaPermissionRequest, WebContextExt, WebViewExt,
};
use wry::{WebContext, WebViewBuilder, WebViewBuilderExtUnix, WebViewExtUnix};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

const APP_NAME: &str = "WhatsApp";

// Logged to the terminal when WHATSAPP_DEBUG=1, to show what WebRTC pieces
// this WebKitGTK build actually exposes to the page.
const DIAGNOSTIC_JS: &str = r#"
if (window.top === window) {
  console.log("[wa-diag] RTCPeerConnection=" + typeof window.RTCPeerConnection +
    " webkitRTCPeerConnection=" + typeof window.webkitRTCPeerConnection +
    " mediaDevices=" + typeof (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) +
    " RTCRtpScriptTransform=" + typeof window.RTCRtpScriptTransform +
    " UA=" + navigator.userAgent);
}
"#;

fn notify(summary: &str, body: &str) {
    // Fire-and-forget; ignore failures (no notification daemon, etc.)
    let _ = Notification::new()
        .appname(APP_NAME)
        .summary(summary)
        .body(body)
        .icon("whatsapp")
        .show();
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(LogicalSize::new(1100.0, 750.0))
        .build(&event_loop)
        .unwrap();

    // Persist cookies/localStorage so you only scan the QR code once
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let data_dir = PathBuf::from(&home).join(".local/share/whatsapp-linux");
    let mut context = WebContext::new(Some(data_dir));

    // Remember the last folder the user saved to
    let default_dir = std::env::var("XDG_DOWNLOAD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(&home).join("Downloads"));
    let last_dir = Rc::new(RefCell::new(default_dir));

    let parent = window.gtk_window().clone();

    let vbox = window.default_vbox().unwrap();
    // Optional override: WHATSAPP_UA="..." to experiment with other user agents
    let user_agent = std::env::var("WHATSAPP_UA").unwrap_or_else(|_| USER_AGENT.to_string());
    let debug = std::env::var("WHATSAPP_DEBUG").is_ok();

    let webview = WebViewBuilder::new_with_web_context(&mut context)
        .with_url("https://web.whatsapp.com")
        .with_user_agent(&user_agent)
        .with_initialization_script(DIAGNOSTIC_JS)
        .build_gtk(vbox)?;

    // ---- WebKitGTK-level configuration (media, WebRTC, notifications) ----
    let wk = webview.webview();

    if let Some(settings) = WebViewExt::settings(&wk) {
        settings.set_enable_media_stream(true); // getUserMedia (mic/camera)
        settings.set_enable_mediasource(true); // MSE for video playback
        settings.set_enable_media_capabilities(true);
        settings.set_enable_encrypted_media(true);
        settings.set_enable_webaudio(true);
        settings.set_enable_webrtc(true); // calls
        settings.set_media_playback_requires_user_gesture(false);
        settings.set_enable_javascript(true);
        // WebKitGTK can silently swap the user agent for certain sites
        settings.set_enable_site_specific_quirks(false);
        settings.set_user_agent(Some(&user_agent));
        settings.set_javascript_can_access_clipboard(true);
        settings.set_enable_write_console_messages_to_stdout(debug);
    }

    // ---- Downloads: "Save as" dialog, defaulting to the shared file's name ----
    if let Some(ctx) = WebViewExt::context(&wk) {
        ctx.connect_download_started(move |_, download| {
            let parent = parent.clone();
            let last_dir = last_dir.clone();
            let failed = Rc::new(Cell::new(false));
            let saved_to: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

            download.connect_decide_destination({
                let saved_to = saved_to.clone();
                move |download, suggested_filename| {
                    let name = if suggested_filename.trim().is_empty() {
                        "download"
                    } else {
                        suggested_filename
                    };

                    let dialog = gtk::FileChooserNative::new(
                        Some("Save file"),
                        Some(&parent),
                        gtk::FileChooserAction::Save,
                        Some("Save"),
                        Some("Cancel"),
                    );
                    dialog.set_do_overwrite_confirmation(true);
                    dialog.set_current_name(name);
                    let _ = dialog.set_current_folder(&*last_dir.borrow());

                    let chosen = if dialog.run() == gtk::ResponseType::Accept {
                        dialog.filename()
                    } else {
                        None
                    };

                    match chosen {
                        Some(path) => {
                            if let Some(dir) = path.parent() {
                                *last_dir.borrow_mut() = dir.to_path_buf();
                            }
                            match gtk::glib::filename_to_uri(&path, None) {
                                Ok(uri) => {
                                    download.set_allow_overwrite(true);
                                    download.set_destination(uri.as_str());
                                    *saved_to.borrow_mut() = Some(path);
                                }
                                Err(_) => download.cancel(),
                            }
                        }
                        None => download.cancel(), // user cancelled
                    }
                    true
                }
            });

            download.connect_failed({
                let failed = failed.clone();
                move |_, _| failed.set(true)
            });

            download.connect_finished(move |_| {
                let path = saved_to.borrow();
                if let Some(p) = path.as_ref() {
                    if failed.get() {
                        notify("Download failed", "The file could not be saved.");
                    } else {
                        notify("Download complete", &p.display().to_string());
                    }
                }
            });
        });
    }

    // Grant mic/camera and notification permission requests automatically
    // (the app only ever loads web.whatsapp.com).
    wk.connect_permission_request(|_, request| {
        if request.is::<UserMediaPermissionRequest>()
            || request.is::<NotificationPermissionRequest>()
        {
            request.allow();
            true
        } else {
            false
        }
    });

    // Native desktop notifications for incoming messages
    wk.connect_show_notification(|_, n| {
        let title = n.title().map(|s| s.to_string()).unwrap_or_default();
        let body = n.body().map(|s| s.to_string()).unwrap_or_default();
        notify(&title, &body);
        true // handled, don't let WebKit show its own
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let _keep_alive = &webview;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
