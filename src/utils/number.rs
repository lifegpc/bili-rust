extern crate json;

use json::JsonValue;

pub trait ToUsize {
    fn to_usize(&self) -> Option<usize>;
}

impl ToUsize for String {
    fn to_usize(&self) -> Option<usize> {
        let r = self.as_str().parse::<usize>();
        match r {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}

impl ToUsize for str {
    fn to_usize(&self) -> Option<usize> {
        let r = self.parse::<usize>();
        match r {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}

impl ToUsize for usize {
    fn to_usize(&self) -> Option<usize> {
        Some(self.clone())
    }
}

impl ToUsize for JsonValue {
    fn to_usize(&self) -> Option<usize> {
        if self.is_number() {
            self.as_usize()
        } else if self.is_string() {
            let r = self.as_str().unwrap().parse::<usize>();
            match r {
                Ok(r) => Some(r),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
