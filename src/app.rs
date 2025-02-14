use anyhow::Result;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Stack};
use crate::pages::{
    AppPage, MainPage, PageOne, PageThree, PageTwo, EditPage
};

pub fn run_app_with_application(app: &Application) -> Result<()> {
    let window = ApplicationWindow::new(app);
    window.set_title("Holy Sheet - 4 Pages Example");
    window.set_default_size(600, 400);

    // Създаваме Stack, което ще държи всички страници
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(200);

    // Създаваме екземпляри на страниците. В конструктора
    // подаваме &stack, за да могат те да превключват към друга страница.
    let main_page = MainPage::new(stack.clone());
    let page_one = PageOne::new(stack.clone());
    let page_two = PageTwo::new(stack.clone());
    let page_three = PageThree::new(stack.clone());
    let edit_page = EditPage::new(stack.clone());

    // Добавяме страниците в Stack
    add_page_to_stack(&stack, &main_page, "main");
    add_page_to_stack(&stack, &page_one, "page_one");
    add_page_to_stack(&stack, &page_two, "page_two");
    add_page_to_stack(&stack, &page_three, "page_three");
    add_page_to_stack(&stack, &edit_page, "edit_page");

    // Слагаме Stack в прозореца
    window.add(&stack);
    window.show_all();

    Ok(())
}

/// Помощна функция за добавяне на страница в Stack.
///
/// * `id` е името, под което ще я достъпваме (`"main"`, `"page_one"`, т.н.).
fn add_page_to_stack(stack: &Stack, page: &impl AppPage, id: &str) {
    stack.add_named(page.widget(), id);
}
