extern crate reqwest;

use reqwest::header::HeaderMap;
use std::collections::HashMap;

/// Convert HeaderMap to HashMap
/// * `hm` - HeaderMap
/// * `map` - HashMap
pub fn header_map_to_hash_map(hm: &HeaderMap, map: &mut HashMap<String, String>) {
    for (k, v) in hm.iter() {
        let k = k.as_str();
        let v = v.to_str();
        match v {
            Ok(v) => {
                map.insert(String::from(k), String::from(v));
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

#[test]
fn test_header_map_to_hash_map() {
    let mut h = HeaderMap::new();
    h.insert("t", "test".parse().unwrap());
    let mut r: HashMap<String, String> = HashMap::new();
    header_map_to_hash_map(&h, &mut r);
    let mut r2: HashMap<String, String> = HashMap::new();
    r2.insert(String::from("t"), String::from("test"));
    assert_eq!(r, r2);
}
