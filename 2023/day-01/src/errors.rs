use std::fmt::Display;

#[derive(Debug)]
pub struct ParseCalibrationValueError<'a> {
    pub invalid_input: &'a str
}

impl<'a> Display for ParseCalibrationValueError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input. Could not read Calibration Value from \"{}\".", self.invalid_input)
    }
}