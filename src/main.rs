use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use gtk4::{glib, prelude::*, ApplicationWindow};
use webkit6::{prelude::*, WebView};

//Code from https://gitlab.gnome.org/World/Rust/webkit6-rs/-/tree/main/examples?ref_type=heads
fn main() -> glib::ExitCode {
    let app = gtk4::Application::new(Some("org.gnome.webkit6-rs.example"), Default::default());
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );
    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let webview = WebView::new();
        webview.load_uri("https://crates.io/");
        window.set_child(Some(&webview));

        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        let inspector = webview.inspector().unwrap();
        inspector.show();

        webview.evaluate_javascript(
            "alert('Hello');",
            None,
            None,
            gtk4::gio::Cancellable::NONE,
            |_result| {},
        );
        webview.evaluate_javascript("42", None, None, gtk4::gio::Cancellable::NONE, |result| {
            match result {
                Ok(value) => {
                    println!("is_boolean: {}", value.is_boolean());
                    println!("is_number: {}", value.is_number());
                    println!("{:?}", value.to_boolean());
                }
                Err(error) => println!("{}", error),
            }
        });
        window.present();
    });
    app.run()
}
