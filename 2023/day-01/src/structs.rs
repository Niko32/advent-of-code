use crate::errors::ParseCalibrationValueError;

#[derive(Clone)]
pub struct ImprovedCalibrationValue(String);

impl ImprovedCalibrationValue {
    fn recover_calibration_value(&self) -> Result<i32, ParseCalibrationValueError> {
        let mut result: String = String::with_capacity(2);

        let mut value_contains_digit = false;

        // Find the first digit in the string
        for c in self.0.chars() {
            if c.is_digit(10) {
                result.push(c);
                value_contains_digit = true;
                break;
            }
        }

        if !value_contains_digit {
            return Err(ParseCalibrationValueError { invalid_input: &self.0 })
        }

        // Find the last digit in the string
        for c in self.0.chars().rev() {
            if c.is_digit(10) {
                result.push(c);
                break;
            }
        }

        Ok(result.parse::<i32>().unwrap())

    }

    fn parse_written_digits(&mut self) -> ImprovedCalibrationValue {
        self.0 = self.0
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        self.clone()
    }
}

pub struct ImprovedCalibrationDocument(Vec<ImprovedCalibrationValue>);

impl From<&str> for ImprovedCalibrationDocument {
    fn from(document_string: &str) -> Self {
        let improved_calibration_values: Vec<ImprovedCalibrationValue> = document_string
            .lines()
            .map(|line| { ImprovedCalibrationValue(line.to_string()) })
            .collect();
        ImprovedCalibrationDocument(improved_calibration_values)
    }
}

impl ImprovedCalibrationDocument {
    pub fn sum_calibration_values(&self) -> i32 {
        let value_ref: &Vec<ImprovedCalibrationValue> = self.0.as_ref();
        value_ref
            .into_iter()
            .map(|improved_calibration_value| { improved_calibration_value.recover_calibration_value().unwrap() })
            .sum()
    }
    pub fn parse_written_digits(& mut self) -> ImprovedCalibrationDocument {
        let new_value = self.0.clone();
        let new_value = new_value
            .into_iter()
            .map(|mut improved_calibration_value| { 
                improved_calibration_value.parse_written_digits()
            })
            .collect::<Vec<ImprovedCalibrationValue>>();
        ImprovedCalibrationDocument(new_value)
    }
}