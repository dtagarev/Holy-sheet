pub mod main_page;
pub mod configurations_page;
pub mod edit_page;
pub mod markdown_viewer;

use gtk::Widget;

/// Common trait for the pages
pub trait AppPage {
    /// Returs the main widget of the page (Box, Grid, etc.),
    /// which is added to the Stack.
    fn widget(&self) -> &Widget;
}

// Rebrand for easy outside access
pub use main_page::MainPage;
pub use configurations_page::ConfigurationsPage;
pub use edit_page::EditPage;