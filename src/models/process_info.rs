/* SPDX-License-Identifier: Apache-2.0 */

//import format module from standard library
use std::fmt;

/*
 *struct ProcessInfo
 *contains process id, name, cpu usage, memory usage and status
*/
pub struct ProcessInfo {
    pub name: String,
    pub id: String,
    pub cpu_usage: String,
    pub status: String,
    pub memory_usage: String,
}

/*
implementing trait Display for SystemInfo
*/
impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<50} {:<5} {:<10} {:<10} {:<10}",
            self.name, self.id, self.cpu_usage, self.memory_usage, self.status
        )
    }
}
