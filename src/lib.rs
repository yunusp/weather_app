pub mod json_schema;

const API_KEY: &'static str = "b744a357c11afb0959f13d6bd1400dbb";
use reqwest::blocking;
pub fn get_req() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn get_coords(city: &str, country: &str) -> Result<(String, String), String> {
    use json_schema::json_location_schema::LocationSchema;
    let req = blocking::get(format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=5&appid={}",
        city, API_KEY
    ))
    .unwrap()
    .json::<LocationSchema>()
    .unwrap();
    let mut long = None;
    let mut lat = None;
    for location in &req {
        if &location.country == country {
            lat = Some(location.lat.to_string());
            long = Some(location.lon.to_string());
        }
    }

    if let (Some(lat), Some(long)) = (lat, long) {
        Ok((lat, long))
    } else {
        Err("No country found".to_string())
    }
}

pub fn get_weather(coords: (String, String)) -> String {
    //TODO: Add error handling
    use json_schema::json_weather_schema::WeatherSchema;
    let req = blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&exclude=hourly,daily,minutely&appid={}",
        coords.0, coords.1, API_KEY
    )).expect("Cant get weather").json::<WeatherSchema>().expect("Cant parse weather");
    // let mut weather = String::new();
    let weather = to_celcius(req.current.temp).to_string();
    weather
}

fn to_celcius(kelvin: f64) -> f64 {
    kelvin - 272.15
}
