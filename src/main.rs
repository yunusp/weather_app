extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (500,500), position: (300,300), title: "Weather", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    name: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_but: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name.text()));
    }
    fn say_goodbye(&self){
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name.text()));
        nwg::stop_thread_dispatch();
    }

}
fn main() {
    // nwg::init().expect("Failed to init Native Windows GUI");
    // nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    // let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    // nwg::dispatch_thread_events();
    use weather_api::get_coords;
    let (x,y) = get_coords("pune", "IN").unwrap();

    println!("{}, {}", x, y);

}
