use anyhow::{Context, Ok, Result, anyhow};
use clap::Parser;
use gdk4::{
    Display, RGBA,
    cairo::{RectangleInt, Region},
    gio::prelude::ListModelExt,
    glib::object::Cast,
    prelude::{DisplayExt, SurfaceExt},
};
use gtk4::{
    Application, CssProvider, PolicyType, STYLE_PROVIDER_PRIORITY_APPLICATION, ScrolledWindow,
    Window,
    gio::{
        ApplicationFlags,
        prelude::{ApplicationExt, ApplicationExtManual},
    },
    glib::ExitCode,
    prelude::{GtkApplicationExt, GtkWindowExt, NativeExt, StyleContextExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::{path::PathBuf, process::exit};
use webkit6::{Settings, WebContext, WebView, prelude::WebViewExt};

use crate::config::Config;

mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(args.config.as_path())?;

    let app = gtk4::Application::new(Some("dev.olaren.overway"), ApplicationFlags::NON_UNIQUE);

    app.connect_activate(move |app| {
        if let Err(err) = activate_app(app, config.clone()) {
            // TODO: Use a gtk window to show the error?
            eprintln!("{}", err);
            app.quit();
        }
    });
    let code = app.run_with_args::<&str>(&[]);

    if code != ExitCode::SUCCESS {
        exit(code.get().into());
    }

    Ok(())
}

fn activate_app(app: &Application, config: Config) -> Result<()> {
    if !gtk4_layer_shell::is_supported() {
        return Err(anyhow!(
            "The wlr_layer_shell wayland extension is not supported on your compositor!"
        ));
    }

    let window = Window::builder().title(&config.title).build();

    // Position the window on the overlay layer with the configured position
    let display = Display::default().context("Cannot get default display")?;
    let monitors = display.monitors();
    if config.monitor >= monitors.n_items() {
        return Err(anyhow!("Invalid monitor index {}", config.monitor));
    }
    let monitor = monitors
        .item(config.monitor)
        .context(format!("Error finding monitor index {}", config.monitor))?;

    let monitor = monitor
        .downcast_ref::<gdk4::Monitor>()
        .context("Cannot downcast monitor")?;

    window.init_layer_shell();
    window.set_monitor(Some(monitor));
    window.set_layer(Layer::Overlay);
    window.set_margin(Edge::Left, config.x);
    window.set_anchor(Edge::Left, true);
    window.set_margin(Edge::Top, config.y);
    window.set_anchor(Edge::Top, true);
    window.set_default_size(config.width, config.height);
    window.set_size_request(config.width, config.height);
    window.show();

    // Disable keyboard / mouse interactions
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None); // TODO: On demand?
    let surface = window.surface().context("Failed to get native surface")?;
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));

    // Make the backgound transparent
    let css_provider = CssProvider::new();
    css_provider.load_from_data("window { background-color: rgba(255, 255, 255, 0); }");
    window
        .style_context()
        .add_provider(&css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Wire up webkitgtk
    let web_context = WebContext::default().context("Failed to init webkit")?;
    web_context.set_cache_model(webkit6::CacheModel::DocumentViewer);

    let wv_settings = Settings::builder()
        .enable_write_console_messages_to_stdout(true)
        .build();

    let web_view = WebView::builder()
        .web_context(&web_context)
        .settings(&wv_settings)
        .zoom_level(config.zoom)
        .build();

    web_view.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 0.0));
    web_view.load_uri(&config.url);

    // We need to put the webview in a ScrolledWindow so we don't get a
    // 1px sized webview widget
    let wv_container = ScrolledWindow::new();
    wv_container.set_policy(PolicyType::Automatic, PolicyType::Automatic);
    wv_container.set_child(Some(&web_view));
    window.set_child(Some(&wv_container));

    // Let the show begin!
    app.add_window(&window);

    Ok(())
}
