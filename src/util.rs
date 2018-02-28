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

pub mod url {
    use std::io;
    use std::process::{Child, Command};

    #[cfg(target_os = "windows")]
    const START_PROGRAM: &str = "explorer";
    #[cfg(target_os = "macos")]
    const START_PROGRAM: &str = "open";
    #[cfg(target_os = "linux")]
    const START_PROGRAM: &str = "xdg-open";

    pub fn open(url: &str) -> Result<Child, io::Error> {
        Command::new(START_PROGRAM).arg(url).spawn()
    }
}
