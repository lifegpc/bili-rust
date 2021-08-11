extern crate subprocess;

use crate::getopt::OptStore;
use crate::i18n::gettext;
use core::time::Duration;
use std::clone::Clone;
use std::net::TcpListener;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

#[derive(Clone, Copy, PartialEq)]
pub enum WebDriverType {
    Chrome,
}

pub struct WebDriverStarter {
    opt: Option<OptStore>,
}

pub struct WebDriverUrlResult {
    pub url: String,
    pub typ: WebDriverType,
    pub cml: Option<Vec<String>>,
}

impl WebDriverUrlResult {
    pub fn new(url: &str, typ: WebDriverType, cml: Option<Vec<String>>) -> WebDriverUrlResult {
        WebDriverUrlResult {
            url: String::from(url),
            typ: typ,
            cml: cml,
        }
    }
}

impl Clone for WebDriverUrlResult {
    fn clone(&self) -> WebDriverUrlResult {
        WebDriverUrlResult {
            url: self.url.clone(),
            typ: self.typ.clone(),
            cml: self.cml.clone(),
        }
    }
}

impl WebDriverStarter {
    pub fn new(opt: Option<OptStore>) -> WebDriverStarter {
        WebDriverStarter { opt }
    }

    pub fn get(&self) -> Option<WebDriverUrlResult> {
        let pb = self.get_prefered_broswer();
        if !pb.is_none() {
            let pb = pb.unwrap();
            let url = self.get_server_url_from_opt(pb);
            if !url.is_none() {
                let url = url.unwrap();
                return Some(WebDriverUrlResult::new(url.as_str(), pb, None));
            }
        }
        let li = self.get_executable(WebDriverType::Chrome);
        for v in li.iter() {
            if self.test_executable(v.clone()) {
                println!("{}\"{}\"", gettext("Found working chrome driver: "), v);
                let port = self.get_port();
                match port {
                    Some(_) => {}
                    None => {
                        println!("{}", gettext("Can not get a working port."));
                        return None;
                    }
                }
                let port = port.unwrap();
                println!("{}{}", gettext("Found working port: "), port);
                let cml = self.get_command_line(WebDriverType::Chrome, v.clone(), port);
                let url = format!("http://127.0.0.1:{}", port);
                return Some(WebDriverUrlResult::new(
                    url.as_str(),
                    WebDriverType::Chrome,
                    Some(cml),
                ));
            }
        }
        None
    }

    fn get_command_line(&self, t: WebDriverType, exe: String, port: u16) -> Vec<String> {
        if t == WebDriverType::Chrome {
            return vec![exe, format!("--port={}", port)];
        }
        [].to_vec()
    }

    fn get_executable(&self, t: WebDriverType) -> Vec<String> {
        if t == WebDriverType::Chrome {
            let mut s = vec![String::from("chromedriver")];
            match &self.opt {
                Some(opt) => {
                    let te = opt.get_option("chromedriver");
                    match te {
                        Some(t) => s.insert(0, t.clone()),
                        None => {}
                    }
                }
                None => {}
            }
            return s;
        }
        [].to_vec()
    }

    fn get_port(&self) -> Option<u16> {
        let mut i: u16 = 1024;
        loop {
            let re = TcpListener::bind(format!("127.0.0.1:{}", i));
            match re {
                Ok(_) => {
                    return Some(i);
                }
                Err(_) => {}
            }
            if i == 65535 {
                break;
            }
            i += 1;
        }
        None
    }

    fn get_prefered_broswer(&self) -> Option<WebDriverType> {
        match &self.opt {
            Some(opt) => {
                if opt.has_option("chrome") {
                    return Some(WebDriverType::Chrome);
                }
                return None;
            }
            None => None,
        }
    }

    fn get_server_url_from_opt(&self, t: WebDriverType) -> Option<String> {
        match &self.opt {
            Some(opt) => {
                if t == WebDriverType::Chrome {
                    return opt.get_option("chromedriver-server");
                }
                return None;
            }
            None => None,
        }
    }

    pub fn kill_server(&self, p: &mut Popen) -> bool {
        match p.kill() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn start_server(&self, cml: Vec<String>) -> Option<Popen> {
        let r = Popen::create(&cml, PopenConfig::default());
        match r {
            Ok(r) => Some(r),
            Err(_) => {
                println!(
                    "{}{:?}",
                    gettext("Can not start server with command line: "),
                    cml
                );
                None
            }
        }
    }

    fn test_executable(&self, exe: String) -> bool {
        let l = vec![exe.as_str(), "-h"];
        let r = Popen::create(
            &l,
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                stderr: Redirection::Pipe,
                ..PopenConfig::default()
            },
        );
        match r {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }
        let mut r = r.unwrap();
        let re = r.wait_timeout(Duration::new(5, 0));
        match re {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }
        let re = re.unwrap();
        if re.is_none() {
            match r.kill() {
                Ok(_) => {}
                Err(_) => {}
            }
            return false;
        }
        let re = re.unwrap();
        if re.success() {
            return true;
        }
        false
    }
}

impl Clone for WebDriverStarter {
    fn clone(&self) -> WebDriverStarter {
        WebDriverStarter {
            opt: self.opt.clone(),
        }
    }
}
