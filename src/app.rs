use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Stack};
use crate::pages::{
    AppPage, MainPage, PageOne, PageThree, PageTwo
};

pub fn run_app() -> Result<()> {
    let app = Application::new(
        Some("com.example.holy-sheet"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Holy Sheet - 4 Pages Example"));
        window.set_default_size(600, 400);

        // Създаваме Stack, което ще държи всички страници
        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);

        // Създаваме екземпляри на страниците. В конструктора
        // подаваме &stack, за да могат те да превключват към друга страница.
        let main_page = MainPage::new(stack.clone());
        let page_one = PageOne::new(stack.clone());
        let page_two = PageTwo::new(stack.clone());
        let page_three = PageThree::new(stack.clone());

        // Добавяме всяка страница в Stack с уникално име (ID).
        // Така после можем да викаме stack.set_visible_child_name("main") например.
        add_page_to_stack(&stack, &main_page, "main");
        add_page_to_stack(&stack, &page_one, "page_one");
        add_page_to_stack(&stack, &page_two, "page_two");
        add_page_to_stack(&stack, &page_three, "page_three");

        // По подразбиране показваме главната страница
        stack.set_visible_child_name("main");

        // Слагаме stack като child на прозореца
        window.set_child(Some(&stack));
        window.show();
    });

    app.run();
    Ok(())
}

/// Помощна функция за добавяне на страница в Stack.
///
/// * `id` е името, под което ще я достъпваме (`"main"`, `"page_one"`, т.н.).
fn add_page_to_stack(stack: &Stack, page: &impl AppPage, id: &str) {
    // page.widget() връща &Widget
    stack.add_named(page.widget(), Some(id));
}
