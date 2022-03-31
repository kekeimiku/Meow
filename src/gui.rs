use fltk::{
    app::{channel, App},
    browser::HoldBrowser,
    button::Button,
    enums::CallbackTrigger,
    input::Input,
    prelude::{BrowserExt, GroupExt, InputExt, WidgetExt},
    window::Window,
};
use sysinfo::{ProcessExt, System, SystemExt};

const WIDGET_WIDTH: i32 = 140;
const WIDGET_HEIGHT: i32 = 50;
const WIDGET_PADDING: i32 = 20;

#[derive(Clone, Copy)]
enum Message {
    Update,
    Select,
    Okkk,
    Filter,
}

pub fn gui() -> Result<(), String> {
    let app = App::default();
    let mut window = Window::default().with_label("LINCE");

    let (sender, receiver) = channel::<Message>();

    let mut filter_input = Input::default()
        .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT / 2)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    //.with_label("Search: ");
    // filter_input.set_value("搜索：");
    filter_input.set_trigger(CallbackTrigger::Changed);
    filter_input.emit(sender, Message::Filter);

    let mut list_browser = HoldBrowser::default()
        .with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 4);
    list_browser.emit(sender, Message::Select);

    let mut update_button = Button::default()
        .with_size(WIDGET_WIDTH / 2, WIDGET_HEIGHT / 2)
        .with_pos(
            WIDGET_PADDING,
            list_browser.y() + list_browser.height() + WIDGET_PADDING,
        )
        .with_label("刷新");
    update_button.emit(sender, Message::Update);
    update_button.activate();

    // update_button.set_color(BLUE);
    // update_button.set_selection_color(SEL_BLUE);
    // update_button.set_label_color(Color::White);

    let mut select_button = Button::default()
        .with_size(WIDGET_WIDTH / 2, WIDGET_HEIGHT / 2)
        .right_of(&update_button, WIDGET_PADDING)
        .with_label("选择");
    select_button.emit(sender, Message::Okkk);
    select_button.deactivate();

    // select_button.set_color(BLUE);
    // select_button.set_selection_color(SEL_BLUE);
    // select_button.set_label_color(Color::White);

    let mut processes_list: Vec<String> = Default::default();

    let mut sys = System::new_all();

    let list_refresh = {
        sys.refresh_processes();
        sys.processes()
            .iter()
            .map(|f| (f.0.to_string(), f.1.name().to_string()))
            .collect::<Vec<(String, String)>>()
    };

    list_refresh
        .iter()
        .map(|f| format!("{} -> {}", f.1, f.0))
        .for_each(|f| processes_list.push(f));

    sender.send(Message::Filter);

    window.set_size(
        list_browser.x() + list_browser.width() + WIDGET_PADDING,
        select_button.y() + update_button.height() + WIDGET_PADDING,
    );
    window.end();
    window.show();
    while app.wait() {
        match receiver.recv() {
            Some(Message::Update) => {
                processes_list.clear();

                let list_refresh = {
                    sys.refresh_processes();
                    sys.processes()
                        .iter()
                        .map(|f| (f.0.to_string(), f.1.name().to_string()))
                        .collect::<Vec<(String, String)>>()
                };

                list_refresh
                    .iter()
                    .map(|f| format!("{} -> {}", f.1, f.0))
                    .for_each(|f| processes_list.push(f));

                sender.send(Message::Filter);
            }

            Some(Message::Okkk) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let index = processes_list
                    .iter()
                    .position(|s| s == &selected_name).ok_or("err")?;
                println!("{}", processes_list[index]);
                // sender.send(Message::Select)
            }

            Some(Message::Select) => {
                if list_browser.value() == 0 {
                    // update_button.deactivate();
                    select_button.deactivate();
                } else {
                    // update_button.activate();
                    select_button.activate();
                }
            }
            Some(Message::Filter) => {
                let prefix = filter_input.value().to_lowercase();
                list_browser.clear();
                for item in &processes_list {
                    if item.to_lowercase().starts_with(&prefix) {
                        list_browser.add(item);
                    }
                }
                sender.send(Message::Select)
            }

            None => {}
        }
    }

    Ok(())
}
