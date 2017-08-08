pub trait ToEnum<T> {
    fn to_enum(&self) -> Vec<T>;
}

macro_rules! try_opt {
    ($value:expr) => {{
        match $value {
            Some(v) => v,
            None => return None,
        }
    }};
}