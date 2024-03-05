use rfd::FileDialog;

fn main() {
    let file = FileDialog::new()
        .add_filter("Game Boy ROM", &["gb"])
        .pick_file();

    match file {
        Some(path) => println!("Selected file: {:?}", path),
        None => println!("No file selected."),
    }
}
