use std::fs;
use directories::ProjectDirs;
use holy_sheet::pages::markdown_viewer::{markdown_to_html, load_markdown_file};

#[test]
fn test_markdown_to_html() {
    let md_text = "# Hello World\nThis is a test.";
    let html = markdown_to_html(md_text);
    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is a test.</p>"));
}

#[test]
fn test_load_markdown_file() {
    let proj_dirs = ProjectDirs::from("com", "markdown", "holy-sheet").unwrap();
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    let file_path = config_dir.join("test.md");

    // Create a test markdown file
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(&file_path, "# Test File\nContent").unwrap();

    let content = load_markdown_file("test.md").unwrap();
    assert_eq!(content, "# Test File\nContent");

    // Clean up
    fs::remove_file(file_path).unwrap();
}
