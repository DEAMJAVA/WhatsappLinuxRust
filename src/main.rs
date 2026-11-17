use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowExtUnix,
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder, WebViewBuilderExtUnix};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WhatsApp")
        .with_inner_size(LogicalSize::new(1100.0, 750.0))
        .build(&event_loop)
        .unwrap();

    // Persist cookies/localStorage so you only scan the QR code once
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let data_dir = std::path::PathBuf::from(home).join(".local/share/whatsapp-linux");
    let mut context = WebContext::new(Some(data_dir));

    let vbox = window.default_vbox().unwrap();
    let _webview = WebViewBuilder::with_web_context(&mut context)
        .with_url("https://web.whatsapp.com")
        .with_user_agent(USER_AGENT)
        .build_gtk(vbox)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
