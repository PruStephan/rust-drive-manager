use std::{fs, io};
use crate::ui::utils::{join_paths, parse_cd_command};

mod utils;

pub fn list_dir(path: String) {
    let paths = fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let mut index = 0;
    for path in &paths {
        let decorator = if index == 0 { "┏" } else if index == paths.len() - 1 { "┗" } else { "┣" };
        println!("{} {}", decorator, path);
        index += 1;
    }
}
pub fn ui_loop(root: String, mut prev_root: Vec<String>){
    print!("{}[2J", 27 as char);
    list_dir(join_paths(&root, &prev_root));
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    if command.trim() == "exit" {
        return;
    }
    let split_command = command.trim().split(" ").collect::<Vec<_>>();
    let l = split_command.len();
    if l != 2 {
        print!("Unknown command. Currently we only support \'cd <dir_name>\'");
        ui_loop(root.to_owned(), prev_root.to_owned());
    }
    else {
        let cur_command = split_command[0];
        let dir = split_command[1];

        match cur_command {
            "cd" => {
                let next_dir = parse_cd_command(dir.to_owned(), root, &mut prev_root);
                ui_loop(next_dir, prev_root.to_owned());
            },
            _ => {
                print!("Unknown command. Currently we only support \'cd <dir_name>\'");
                ui_loop(root.to_owned(), prev_root.to_owned())
            }
        }
    }
}

