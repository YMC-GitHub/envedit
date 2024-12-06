use winres;
/// use winres to set program file icon in window
///
/// use "icon.ico" in project root
fn set_program_file_icon() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");

    res.compile().unwrap();
}

fn main() {
    set_program_file_icon();
}
