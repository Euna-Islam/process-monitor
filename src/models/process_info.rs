use std::fmt;

pub struct ProcessInfo {
    pub name: String,
    pub id: String,
    pub cpu_usage: String,
    pub status: String,
    pub memory_usage: String,
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<50} {:<5} {:<10} {:<10} {:<10}",
            self.name, self.id, self.cpu_usage, self.memory_usage, self.status
        )
    }
}
