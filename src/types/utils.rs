use num_format::{Locale, ToFormattedString};
use num_traits::ToPrimitive;
use std::any::TypeId;
use std::fs;

pub fn num<T>(n: T) -> String
where
    T: ToPrimitive + 'static,
{
    if TypeId::of::<T>() == TypeId::of::<f32>() || TypeId::of::<T>() == TypeId::of::<f64>() {
        // Convert to f64 (it will succeed if T is f32 or f64)
        let float_val = n.to_f64().unwrap();

        // Round the number to 2 decimal places.
        // (Multiplying and dividing by 100 handles the rounding correctly.)
        let rounded = (float_val * 100.0).round() / 100.0;

        // Format the rounded value with exactly 2 decimals.
        // (For example, 12345.0 becomes "12345.00".)
        let formatted = format!("{:.2}", rounded);
        // Split the formatted string into integer and fractional parts.
        let parts: Vec<&str> = formatted.split('.').collect();
        let int_part = parts[0];
        let frac_part = parts.get(1).unwrap_or(&"00");

        // Insert commas into the integer part.
        // If the number is negative, remove the '-' for formatting and re-add it.
        let (sign, digits) = if int_part.starts_with('-') {
            ("-", &int_part[1..])
        } else {
            ("", int_part)
        };

        // Parse the digit portion as a u64 to use the num-format crate.
        // (We assume here that the absolute value fits into a u64.)
        let int_val: u64 = digits.parse().unwrap_or(0);
        let formatted_integer = int_val.to_formatted_string(&Locale::en);

        // Recombine the formatted parts.
        format!("{}{}.{}", sign, formatted_integer, frac_part)
    } else {
        // For non-float types (assumed to be integer types),
        // we format them with commas and no decimals.
        if let Some(int_val) = n.to_u64() {
            int_val.to_formatted_string(&Locale::en)
        } else {
            String::from("Unsupported type")
        }
    }
}

pub enum StatType {
    Positive,
    Negative,
    Ratio
}

pub fn stat<T>(stat: T, _type: StatType) -> String
where 
    T: ToPrimitive + 'static 
{
    let formatted = num(stat);

    match _type {
        StatType::Positive => format!("<green>{formatted}</green>"),
        StatType::Negative => format!("<red>{formatted}</red>"),
        StatType::Ratio => format!("<gold>{formatted}</gold>"),
    }
}

pub fn load_image(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to load image")
}