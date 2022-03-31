extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (500,500), position: (300,300), title: "Weather", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid_layout: nwg::GridLayout,

    #[nwg_control(text: "pune", size: (280, 35), focus: true)]
    #[nwg_layout_item(layout: grid_layout, row:0, col: 0)]
    city: nwg::TextInput,

    #[nwg_control(text: "IN", size: (280, 35), focus: true)]
    #[nwg_layout_item(layout: grid_layout, row:0, col: 1)]
    country: nwg::TextInput,

    #[nwg_control(text: "0", size: (280, 35))]
    #[nwg_layout_item(layout: grid_layout, row:1, col: 1)]
    lat: nwg::Label,
    
    #[nwg_control(text: "0", size: (280, 35))]
    #[nwg_layout_item(layout: grid_layout, row:1, col: 2)]
    long: nwg::Label,

    #[nwg_control(text: "get", size: (280, 70))]
    #[nwg_layout_item(layout: grid_layout, row:0, col: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::get] )]
    get: nwg::Button,
}

impl BasicApp {
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }
    fn get(&self) {
        use weather_api::get_coords;
        let (x, y) = get_coords(&self.city.text(), &self.country.text()).unwrap();
        self.lat.set_text(&x);
        self.long.set_text(&y);
    }
}
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
    // use weather_api::get_coords;
    // let (x,y) = get_coords("pune", "IN").unwrap();

    // println!("{}, {}", x, y);
}
