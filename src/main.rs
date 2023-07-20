mod ui;

fn main() {
    let root = String::from("stephan");
    let prev_root = vec![String::from("/Users")];
    ui::ui_loop(root.to_owned(), prev_root);
}
