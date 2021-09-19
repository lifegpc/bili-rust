extern crate json;

use json::JsonValue;

pub trait ToStr {
    /// Convert to `&str`
    fn to_str(&self) -> Option<&str>;
}

impl ToStr for String {
    fn to_str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ToStr for &str {
    fn to_str(&self) -> Option<&str> {
        Some(&self.clone())
    }
}

impl ToStr for JsonValue {
    fn to_str(&self) -> Option<&str> {
        self.as_str()
    }
}
