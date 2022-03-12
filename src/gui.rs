use fltk::{app::*, browser::*, button::*, enums::*, input::*, prelude::*, window::*};

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
}

pub fn gui() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default().with_label("demo1");

    let (sender, receiver) = channel::<Message>();

    let mut filter_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filter prefix:");
    filter_input.set_trigger(CallbackTrigger::Changed);
    filter_input.emit(sender, Message::Filter);

    let mut list_browser = HoldBrowser::default()
        .with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 4);
    list_browser.emit(sender, Message::Select);

    let name_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
            list_browser.y(),
        )
        .with_label("Name:");

    let surname_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&name_input, WIDGET_PADDING)
        .with_label("Surname:");

    let mut create_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            WIDGET_PADDING,
            list_browser.y() + list_browser.height() + WIDGET_PADDING,
        )
        .with_label("Create");
    create_button.emit(sender, Message::Create);

    let mut update_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&create_button, WIDGET_PADDING)
        .with_label("Update");
    update_button.emit(sender, Message::Update);
    update_button.deactivate();

    let mut delete_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&update_button, WIDGET_PADDING)
        .with_label("Delete");
    delete_button.emit(sender, Message::Delete);
    delete_button.deactivate();

    let mut model = vec![
        // "Babbage, Charles".to_string(),
        // "Lovelace, Ada".to_string(),
        // "Turing, Alan".to_string(),
    ];
    sender.send(Message::Filter);

    let formatted_name = || format!("{}no input {}", surname_input.value(), name_input.value());

    wind.set_size(
        name_input.x() + name_input.width() + WIDGET_PADDING,
        create_button.y() + create_button.height() + WIDGET_PADDING,
    );
    wind.end();
    wind.show();
    while app.wait() {
        match receiver.recv() {
            Some(Message::Create) => {
                model.push(formatted_name());
                sender.send(Message::Filter);
            }
            Some(Message::Update) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let index = model.iter().position(|s| s == &selected_name).unwrap();
                model[index] = formatted_name();
                sender.send(Message::Filter);
            }
            Some(Message::Delete) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let index = model.iter().position(|s| s == &selected_name).unwrap();
                model.remove(index);
                sender.send(Message::Filter);
                sender.send(Message::Select)
            }
            Some(Message::Select) => {
                if list_browser.value() == 0 {
                    update_button.deactivate();
                    delete_button.deactivate();
                } else {
                    update_button.activate();
                    delete_button.activate();
                }
            }
            Some(Message::Filter) => {
                let prefix = filter_input.value().to_lowercase();
                list_browser.clear();
                for item in &model {
                    if item.to_lowercase().starts_with(&prefix) {
                        list_browser.add(item);
                    }
                }
                sender.send(Message::Select)
            }
            None => {}
        }
    }
}