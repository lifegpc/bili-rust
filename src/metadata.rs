use std::clone::Clone;

pub struct NoInTotal {
    _no: usize,
    _total: usize,
}

impl Clone for NoInTotal {
    fn clone(&self) -> NoInTotal {
        NoInTotal {
            _no: self._no.clone(),
            _total: self._total.clone(),
        }
    }
}
