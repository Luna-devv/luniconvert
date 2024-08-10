#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use std::collections::HashMap;
use regex::Regex;

#[derive(Copy, Clone)]
struct Conversion {
    factor: f64,
    offset: f64, // Offset for linear transformation (e.g., Fahrenheit)
}

#[napi]
pub struct Converter {
    conversions: HashMap<String, Conversion>,
    prefixes: HashMap<String, f64>,
}

#[napi]
impl Converter {
    #[napi(constructor)]
    pub fn new() -> Self {
        let mut conversions = HashMap::new();

        // Distance units (base: meter)
        conversions.insert("m".to_string(), Conversion { factor: 1.0, offset: 0.0 });
        conversions.insert("mile".to_string(), Conversion { factor: 1609.34, offset: 0.0 });
        conversions.insert("yard".to_string(), Conversion { factor: 0.9144, offset: 0.0 });
        conversions.insert("foot".to_string(), Conversion { factor: 0.3048, offset: 0.0 });
        conversions.insert("inch".to_string(), Conversion { factor: 0.0254, offset: 0.0 });

        // Temperature units (base: Celsius)
        conversions.insert("C".to_string(), Conversion { factor: 1.0, offset: 0.0 });
        conversions.insert("K".to_string(), Conversion { factor: 1.0, offset: -273.15 });
        conversions.insert("F".to_string(), Conversion { factor: 5.0 / 9.0, offset: -32.0 });

        let mut prefixes = HashMap::new();
        prefixes.insert("n".to_string(), 1e-9);    // nano
        prefixes.insert("μ".to_string(), 1e-6);    // micro
        prefixes.insert("m".to_string(), 1e-3);    // milli
        prefixes.insert("c".to_string(), 1e-2);    // centi
        prefixes.insert("".to_string(), 1.0);      // no prefix (base)
        prefixes.insert("k".to_string(), 1e3);     // kilo
        prefixes.insert("M".to_string(), 1e6);     // mega
        prefixes.insert("G".to_string(), 1e9);     // giga

        Self { conversions, prefixes }
    }

    fn extract_value(&self, text: &str) -> Option<f64> {
        let re = Regex::new(r"\d+").unwrap();

        if let Some(matched) = re.find(text) {
            Some(matched.as_str().to_string().parse::<f64>().unwrap())
        } else {
            None
        }
    }

    fn extract_unit(&self, text: &str) -> Option<String> {
        let re = Regex::new(r"[a-zA-Z]+").unwrap();

        if let Some(matched) = re.find(text) {
            Some(matched.as_str().to_string())
        } else {
            None
        }
    }

    fn parse_unit(&self, unit: &str) -> Result<(f64, f64, String)> {
        let mut prefix = "";
        let mut base_unit = unit;

        if base_unit == unit {
            if let Some(conversion) = self.conversions.get(unit) {
                return Ok((conversion.factor, conversion.offset, unit.to_string()));
            }
        }

        for p in self.prefixes.keys() {
            if unit.starts_with(p) && !p.is_empty() {
                prefix = p;
                base_unit = &unit[p.len()..];
                break;
            }
        }

        if let Some(conversion) = self.conversions.get(base_unit) {
            let prefix_factor = self.prefixes.get(prefix).unwrap_or(&1.0);
            let factor = conversion.factor * prefix_factor;
            let offset = conversion.offset;
            Ok((factor, offset, base_unit.to_string()))
        } else {
            Err(Error::new(Status::InvalidArg, format!("Invalid unit: {}", unit)))
        }
    }

    fn format_number(value: f64) -> String {
        let formatted = format!("{:.2}", value);
        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
    }

    #[napi]
    pub fn convert(&self, input: String) -> Result<String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 1 || parts.len() > 3 {
            return Err(Error::new(Status::InvalidArg, "Invalid input format".to_string()));
        }

        let value = self.extract_value(&parts[0]).ok_or_else(|| {
            Error::new(Status::InvalidArg, "Invalid number format".to_string())
        })?;

        let from_unit = self.extract_unit(&parts[0]).ok_or_else(|| {
            Error::new(Status::InvalidArg, "Invalid input format".to_string())
        })?;

        let to_unit = if parts.len() == 3 {
            parts[2]
        } else {
            &from_unit
        };

        let (from_factor, from_offset, _) = self.parse_unit(&from_unit)?;
        let (to_factor, to_offset, _) = self.parse_unit(to_unit)?;


        let value_in_base = (value + from_offset) * from_factor;
        let result = (value_in_base / to_factor) - to_offset;

        Ok(format!("{} {}", Self::format_number(result), to_unit))
    }

    #[napi]
    pub fn add_conversion(&mut self, unit: String, factor: f64, offset: f64) {
        self.conversions.insert(unit, Conversion { factor, offset });
    }
}