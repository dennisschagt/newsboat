use crate::tui::bridged::draw_tui_window;
use libnewsboat::tui;
use std::pin::Pin;

use self::bridged::IWindow;

// cxx doesn't allow to share types from other crates, so we have to wrap it
// cf. https://github.com/dtolnay/cxx/issues/496
struct Tui(tui::TuiType);

#[cxx::bridge(namespace = "newsboat::tui::bridged")]
mod bridged {
    extern "Rust" {
        type Tui;

        fn create() -> Box<Tui>;

        fn init(tui: &mut Tui);
        fn exit(tui: &mut Tui);
        fn draw(tui: &mut Tui);
        fn wait_for_event(tui: &mut Tui) -> String;

        fn register_window(window: Pin<&mut IWindow>);
    }

    unsafe extern "C++" {
        include!("libnewsboat-ffi/include/tuiwindow.h");

        fn draw_tui_window();

        type IWindow;

        fn get_title(self: Pin<&mut IWindow>) -> &CxxString;
    }
}

fn create() -> Box<Tui> {
    Box::new(Tui(tui::TuiType::new()))
}

fn init(tui: &mut Tui) {
    tui.0.init();
}

fn exit(tui: &mut Tui) {
    tui.0.exit();
}

fn draw(tui: &mut Tui) {
    tui.0.draw();
    draw_tui_window();
}

fn wait_for_event(tui: &mut Tui) -> String {
    tui.0.wait_for_event()
}

fn register_window(window: Pin<&mut IWindow>) {
    let title = window.get_title();
    println!("title: {}", title);
}
