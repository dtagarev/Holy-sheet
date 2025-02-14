pub mod main_page;
pub mod page_one;
pub mod page_two;
pub mod page_three;
pub mod edit_page;
pub mod markdown_viewer;

use gtk::Widget;

/// Общ trait за страниците
pub trait AppPage {
    /// Връща главния widget на страницата (Box, Grid и т.н.),
    /// който се поставя в Stack.
    fn widget(&self) -> &Widget;
}

// Ребрандираме ги за по-лесен достъп отвън
pub use main_page::MainPage;
pub use page_one::PageOne;
pub use page_two::PageTwo;
pub use page_three::PageThree;
pub use edit_page::EditPage;