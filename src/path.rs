use std::env;
use std::path::Path;
use std::path::PathBuf;

/// Get executable location, if not found, return None
pub fn get_exe_path() -> Option<PathBuf> {
    let re = env::current_exe();
    match re {
        Ok(pa) => {
            let mut p = pa.clone();
            p.pop();
            Some(p)
        }
        Err(_) => None,
    }
}

/// Get executable location, if not found, return current directory (./)
pub fn get_exe_path_else_current() -> PathBuf {
    let re = env::current_exe();
    match re {
        Ok(pa) => {
            let mut p = pa.clone();
            p.pop();
            p
        }
        Err(_) => {
            let p = Path::new("./");
            p.to_path_buf()
        }
    }
}

/// Convert `&Path` to `&str`
/// * `p` - Origin path
pub fn path_to_str(p: &Path) -> &str {
    let f = p.to_str();
    match f {
        Some(n) => n,
        None => "<Convert Error>",
    }
}
