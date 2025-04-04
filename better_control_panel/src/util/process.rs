use std::process::{Child, Command};

///
/// std::process::Command extension trait
///
pub trait CommandExt {
    ///
    /// Windows: Spawns the command without window.
    /// Other platforms: Same as [`Command::spawn`].
    ///
    fn spawn_without_window(&mut self) -> std::io::Result<Child>;
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

impl CommandExt for Command {
    fn spawn_without_window(&mut self) -> std::io::Result<Child> {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW);
        }
        self.spawn()
    }
}
