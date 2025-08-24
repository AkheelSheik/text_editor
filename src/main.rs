mod editor;
use editor::Editor;
#[warn(clippy::all, clippy::pedantic)]

fn main() {
    // let args: Vec<string? = std::env::args().collect()
    // if let some(first_arg) = args.get(1) {
        
    // } else {
    //     println!("No arg given");
    // }
    let mut editor = Editor::new();
    editor.run();
}