use std::ops::Add;

pub fn join_paths(dir: &String, prev_paths: &Vec<String>) -> String {
    prev_paths.join("/").add(&*String::from("/")).add(dir)
}

pub fn parse_cd_command(next_dir: String, cur_dir: String, other_path: &mut Vec<String>) -> String {
    match next_dir.as_str(){
        ".." => {
            let prev_dir = other_path.pop();
            match prev_dir {
                Some(x) => x,
                _ => String::new()
            }
        }
        _ => {
            other_path.push(cur_dir);
            next_dir
        }
    }
}
