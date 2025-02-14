use anyhow::Result;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, StyleContext, Stack};
use crate::pages::{
    AppPage, MainPage, PageOne, PageThree, ConfigurationsPage, EditPage
};

pub fn run_app_with_application(app: &Application) -> Result<()> {
    let window = ApplicationWindow::new(app);
    window.set_title("Holy Sheet");
    window.set_default_size(600, 400);

    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("pages/style/style.css")).expect("Failed to load CSS");
    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Create a Stack that will hold all the pages
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(200);

    // Create instances of the pages. In the constructor
    // we pass &stack, so they can switch to another page.
    let edit_page = EditPage::new(stack.clone(), app.clone());
    let main_page = MainPage::new(stack.clone());
    let page_one = PageOne::new(stack.clone());
    let configurations_page = ConfigurationsPage::new(stack.clone(), edit_page.clone());
    let page_three = PageThree::new(stack.clone());

    // Add the pages to the Stack
    add_page_to_stack(&stack, &main_page, "main");
    add_page_to_stack(&stack, &page_one, "page_one");
    add_page_to_stack(&stack, &configurations_page, "configurations_page");
    add_page_to_stack(&stack, &page_three, "page_three");
    add_page_to_stack(&stack, &edit_page, "edit_page");

    window.add(&stack);
    window.show_all();

    Ok(())
}

fn add_page_to_stack(stack: &Stack, page: &impl AppPage, id: &str) {
    stack.add_named(page.widget(), id);
    if id == "edit_page" {
        stack.set_child_name(page.widget(), Some("edit_page"));
    }
}
