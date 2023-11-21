use std::fmt;

pub struct SystemInfo {
    pub host: String,
    pub name: String
}

//source: https://stackoverflow.com/a/36439447
impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System Host: {}\nSystem Name: {}", self.host, self.name)
    }
}