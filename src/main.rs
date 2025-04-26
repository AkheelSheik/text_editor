mod editor;
use editor::Editor;
#[warn(clippy::all, clippy::pedantic)]

fn main() {
    let mut editor = Editor::new();
    editor.run();
}