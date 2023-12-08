/* SPDX-License-Identifier: Apache-2.0 */

/*
 * User commands
 */
pub enum UserCommand {
    Help, GetSystemInfo, ListProcess, SortProcessByMemory, SortProcessByCpu, Exit, Invalid
}

impl UserCommand  {
    pub fn convert_str_to_cmd(cmd_str: &str) -> UserCommand {
            if let Ok(num) = cmd_str.parse::<i32>() {
                match num {
                    0 => UserCommand::Help,
                    1 => UserCommand::GetSystemInfo,
                    2 => UserCommand::ListProcess,
                    3 => UserCommand::SortProcessByMemory,
                    4 => UserCommand::SortProcessByCpu,
                    5 => UserCommand::Exit,
                    _ => UserCommand::Invalid
                }
            } else {
                UserCommand::Invalid
            }
    }
}