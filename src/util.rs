use ::rocket::http::RawStr;
use std::ascii::AsciiExt;

pub trait StringUtil {
    fn is_ascii(&self) -> bool;
}

impl<'a> StringUtil for &'a RawStr {
    fn is_ascii(&self) -> bool {
        self.chars().all(|c| c.is_ascii())
    }
}