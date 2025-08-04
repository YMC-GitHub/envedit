#[cfg(target_os = "windows")]
use winres;
/// use winres to set program file icon in window
///
/// use "icon.ico" in project root
#[cfg(target_os = "windows")]
fn set_program_file_icon() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");

    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn set_program_file_icon() {
    // No icon setting needed for non-Windows platforms
}
fn main() {
    set_program_file_icon();
}
