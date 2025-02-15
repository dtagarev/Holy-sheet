use std::fs;
use directories::ProjectDirs;
use holy_sheet::pages::edit_page::save_file;

#[test]
fn test_save_file() {
    let proj_dirs = ProjectDirs::from("com", "example", "holy-sheet").unwrap();
    let config_dir = proj_dirs.config_dir().join("cheatsheets");
    let file_path = config_dir.join("test_save.md");

    // Save a test file
    save_file("test_save", "# Test Save\nContent").unwrap();

    // Verify the file content
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "# Test Save\nContent");

    // Clean up
    fs::remove_file(file_path).unwrap();
}
