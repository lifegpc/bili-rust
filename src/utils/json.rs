extern crate json;

use json::JsonValue;

/// Return the first value which is not null by using the key in keys.
/// * `obj` - JSON Object
/// * `keys` - Keys of object key
/// # Examples
/// ```
/// let j = json::object! {"a": "b", "d": null};
/// let v = jv_multikey_value(j, vec!["d", "a"]);
/// assert_eq!(v.unwrap(), "b");
/// ```
pub fn jv_multikey_value<'a>(obj: &'a JsonValue, keys: Vec<&'a str>) -> Option<&'a JsonValue> {
    for k in keys {
        let v = &obj[k];
        if !v.is_null() {
            return Some(v);
        }
    }
    None
}

#[test]
fn test_jv_multikey_value() {
    let j = json::object! {"a": "b", "d": null};
    let v = jv_multikey_value(&j, vec!["d", "a"]);
    assert_eq!(v.unwrap(), "b");
}
