use windows::{
  Win32::System::Console::*, Win32::System::Threading::*,
};

pub fn main() -> Result<(), String> {
  unsafe {
    if AttachConsole(GetCurrentProcessId()).as_bool() || AllocConsole().as_bool() {
      println!("test");
    }
  }

  return Ok(());
}