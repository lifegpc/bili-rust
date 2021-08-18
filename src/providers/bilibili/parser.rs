extern crate html_parser;

use crate::i18n::gettext;
use html_parser::Dom;
use html_parser::Element;
use html_parser::Node;
use std::clone::Clone;
use std::collections::HashMap;

/// Get string from script in HTML.
pub struct HTMLDataInJS {
    /// Key and data map
    pub maps: HashMap<String, String>,
}

impl HTMLDataInJS {
    pub fn new() -> HTMLDataInJS {
        HTMLDataInJS {
            maps: HashMap::new(),
        }
    }

    fn get_text_in_node(n: &Element) -> Option<String> {
        let mut s = String::from("");
        for i in n.children.iter() {
            match i {
                Node::Element(_) => {return None;}
                Node::Text(t) => {
                    s += t;
                }
                Node::Comment(_) => {
                    return None;
                }
            }
        }
        Some(s)
    }

    fn iter(&mut self, list: &Vec<Node>, keys: Vec<&str>) -> bool {
        for n in list.iter() {
            match n {
                Node::Element(e) => {
                    if e.name == "script" {
                        let t = Self::get_text_in_node(e);
                        if t.is_none() {
                            return false;
                        }
                        let t = t.unwrap();
                        for k in keys.iter() {
                            let ke = format!("{}=", k);
                            let re = t.find(ke.as_str());
                            if !re.is_none() && re.unwrap() == 0 {
                                let v = &t[ke.len()..t.len()];
                                self.maps.insert(String::from(*k), String::from(v));
                            }
                        }
                    } else {
                        let r = self.iter(&e.children, keys.clone());
                        if !r {
                            return false;
                        }
                    }
                }
                Node::Text(_) => {}
                Node::Comment(_) => {}
            }
        }
        true
    }
    /// Parse HTML and get JSON data from script tag in HTML
    /// * `html` - HTML content
    /// * `keys` - Keys.
    /// # Examples
    /// ```
    /// let html = "<script>key=value</script>";
    /// let a = HTMLDataInJS::new();
    /// if a.parse(html, vec!["key"]) {
    ///     let s = a.maps.get("key").unwrap();
    ///     println!("{}", s); // value
    /// }
    /// ```
    pub fn parse(&mut self, html: &str, keys: Vec<&str>) -> bool {
        let doc = Dom::parse(html);
        match doc {
            Ok(_) => {}
            Err(e) => {
                println!("{}\"{}\"", gettext("Can not parse HTML: "), e);
                return false;
            }
        }
        let doc = doc.unwrap();
        self.iter(&doc.children, keys)
    }
}

impl Clone for HTMLDataInJS {
    fn clone(&self) -> HTMLDataInJS {
        HTMLDataInJS {
            maps: self.maps.clone(),
        }
    }
}

#[test]
fn test_html_data_in_js() {
    let mut maps: HashMap<String, String> = HashMap::new();
    maps.insert(String::from("test"), String::from("kev=ad"));
    maps.insert(String::from("ok"), String::from("fuck"));
    let mut js = HTMLDataInJS::new();
    assert_eq!(true, js.parse("<script>test=kev=ad</script><div><script>ok=fuck</script></div>", vec!["test", "ok"]));
    assert_eq!(maps, js.maps);
}
