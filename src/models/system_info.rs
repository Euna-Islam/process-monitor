/* SPDX-License-Identifier: Apache-2.0 */

//import format module from standard library
use std::fmt::{self};

/*
 *struct SystemInfo
 *contains system host and name
*/
pub struct SystemInfo {
    pub host: String,
    pub name: String,
}
/*
implementing trait Display for SystemInfo
*/
impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System Host: {}\nSystem Name: {}", self.host, self.name)
    }
}
