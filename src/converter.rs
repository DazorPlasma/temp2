pub enum Temperature {
    Celsius,
    Fahrenheit,
}

pub fn temperature_convert(temp: &f64, temp_type: Temperature) -> f64 {
    match temp_type {
        Temperature::Celsius => temp * 1.8 + 32.0,
        Temperature::Fahrenheit => (temp - 32.0) / 1.8,
    }
}
