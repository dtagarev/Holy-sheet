use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Button, Label};
use directories::ProjectDirs;
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use wry::WebViewBuilder;
use gtk4::builders::WindowBuilder;

#[derive(Default)]
struct App {
    window: Option<Window>,
    webview: Option<wry::WebView>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        let html_content = match load_markdown_file("test.md") {
            Ok(content) => markdown_to_html(&content),
            Err(_) => "<p>File does not exist</p>".to_string(),
        };
        let webview = WebViewBuilder::new()
            .with_url(&format!("data:text/html,{}", html_content))
            .build(&window)
            .unwrap();

        self.window = Some(window);
        self.webview = Some(webview);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        #[cfg(target_os = "linux")]
        while gtk::events_pending() {
          gtk::main_iteration_do(false);
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {}
}

pub fn run_markdown_viewer(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    gtk4::init().expect("Failed to initialize GTK");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

fn load_markdown_file(filename: &str) -> Result<String, String> {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet")
        .ok_or("Could not locate config directory")?;
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    let file_path = config_dir.join(filename);

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read {:?}: {}", file_path, e))?;
    Ok(content)
}

fn markdown_to_html(md_text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(md_text, options);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    let css = r#"
    <style>
        body {
            font-family: sans-serif;
            margin: 1rem;
        }
        h1, h2, h3 {
            color: #2d76d9;
        }
        code {
            background: #f4f4f4;
            padding: 2px 4px;
            border-radius: 4px;
        }
    </style>
    "#;

    format!(
        r#"<!DOCTYPE html>
<html>
<head>{css}</head>
<body>
{content}
</body>
</html>
"#,
        css = css,
        content = html_buf
    )
}