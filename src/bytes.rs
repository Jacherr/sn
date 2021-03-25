use core::{fmt, fmt::Debug};
use std::borrow::Cow;
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> From<&'a str> for Bytes<'a> {
    fn from(s: &'a str) -> Self {
        Self(s.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(s: &'a [u8]) -> Self {
        Self(s)
    }
}

impl<'a> Debug for Bytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bytes").field(&self.as_utf8_str()).finish()
    }
}

impl<'a> Bytes<'a> {
    pub fn as_utf8_str(&self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }
}
