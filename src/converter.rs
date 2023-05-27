use std::fmt;

pub struct Celsius(pub f64);
pub struct Fahrenheit(pub f64);

impl Celsius {
    pub fn to_fahrenheit(&self) -> Fahrenheit {
        let val = self.0 * 1.8 + 32.0;
        Fahrenheit((val * 100000.0).round() / 100000.0)
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> Celsius {
        let val = (self.0 - 32.0) / 1.8;
        Celsius((val * 100000.0).round() / 100000.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fahren_to_celsius1() {
        let fah = Fahrenheit(32.0);
        let cel = Celsius(0.0);
        assert_eq!(fah.to_celsius().0, cel.0);
    }

    #[test]
    fn fahren_to_celsius2() {
        let fah = Fahrenheit(221.0);
        let cel = Celsius(105.0);
        assert_eq!(fah.to_celsius().0, cel.0);
    }

    #[test]
    fn celsius_to_fahren1() {
        let cel = Celsius(57.0);
        let fah = Fahrenheit(134.6);
        assert_eq!(cel.to_fahrenheit().0, fah.0);
    }

    #[test]
    fn celsius_to_fahren2() {
        let cel = Celsius(2199.0);
        let fah = Fahrenheit(3990.2);
        assert_eq!(cel.to_fahrenheit().0, fah.0);
    }

    #[test]
    fn back_and_forth1() {
        let cel = Celsius(12345.6789);
        assert_eq!(cel.0, cel.to_fahrenheit().to_celsius().0);
    }

    #[test]
    fn back_and_forth2() {
        let cel = Celsius(7345745457.01);
        assert_eq!(cel.0, cel.to_fahrenheit().to_celsius().0);
    }
}