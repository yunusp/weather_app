// #![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (500,500), position: (300,300), title: "Weather", flags: "WINDOW|VISIBLE|RESIZABLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid_layout: nwg::GridLayout,

    #[nwg_control(text: "City:", size: (500, 15))]
    #[nwg_layout_item(layout: grid_layout, row:0, col: 0)]
    city_label: nwg::Label,
    #[nwg_control(text: "pune", size: (280, 35), focus: true)]
    #[nwg_layout_item(layout: grid_layout, row:1, col: 0)]
    city: nwg::TextInput,

    #[nwg_control(text: "Country:", size: (500, 15))]
    #[nwg_layout_item(layout: grid_layout, row:0, col: 2)]
    country_label: nwg::Label,
    #[nwg_control(text: "IN", size: (280, 35), focus: true)]
    #[nwg_layout_item(layout: grid_layout, row:1, col: 2)]
    country: nwg::TextInput,

    #[nwg_control(text: "0", size: (500, 15))]
    #[nwg_layout_item(layout: grid_layout, row:2, col: 0)]
    lat: nwg::Label,

    #[nwg_control(text: "0", size: (500, 15))]
    #[nwg_layout_item(layout: grid_layout, row:2, col: 2)]
    long: nwg::Label,

    #[nwg_control(text: "0", size: (500, 15))]
    #[nwg_layout_item(layout: grid_layout, row:3, col: 2)]
    temp: nwg::Label,

    #[nwg_control(text: "get", size: (1, 1))]
    #[nwg_layout_item(layout: grid_layout, row:1, col: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::set_temp] )]
    get: nwg::Button,
}

impl BasicApp {
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }
    fn set_coords(&self) {
        use weather_api::get_coords;
        let (x, y) = get_coords(&self.city.text(), &self.country.text()).unwrap_or_default();
        self.lat.set_text(&x);
        self.long.set_text(&y);
    }
    #[allow(dead_code)]
    fn handle_resize(&self) {
        let (w, h) = &self.window.size();
        let _ = &self.get.set_position((*w / 2) as i32, (*h / 2) as i32);
    }
    fn set_temp(&self) {
        use weather_api::get_weather;
        let temp = get_weather((self.lat.text().to_string(), self.long.text().to_string()));
        self.temp.set_text(&temp);
    }

    // fn handle_click(&self) {
    //     self.set_temp();
    //     self.set_coords()
    // }
}
fn main() {
    println!("hello");
    use weather_api::get_weather;
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
    println!("hello {}", get_weather(("33.44".to_string(),"-94.04".to_string())));
}
