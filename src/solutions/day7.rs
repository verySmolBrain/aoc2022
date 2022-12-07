use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

struct Directory {
    name: String,
    children: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Directory) {
        self.children.push(child);
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn size(&self) -> i32 {
        let mut size = 0;
        size += self.files.iter()
            .fold(0, |acc, file| acc + file.size());
        size += self.children.iter()
            .fold(0, |acc, child| acc + child.size());
        
        return size;
    }

    fn get_child(&self, name: &str) -> Option<&mut Directory> {
        for child in &self.children {
            if child.name == name {
                return Some(&mut child);
            }
        }

        for child in &self.children {
            if let Some(dir) = child.get_child(name) {
                return Some(dir);
            }
        }

        return None;
    }
}

struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: String, size: i32) -> File {
        File { name, size }
    }

    fn size(&self) -> i32 {
        self.size
    }
}

fn part1(input: &str) -> i32 {
    let mut root = Directory::new("root".to_string());
    let cur_dir = &mut root;

    for line in input.lines() {
        if line == "$ cd .." {
            continue;
        } else if line.starts_with("$ cd") {
            let name = line[5..].to_string();
            cur_dir = cur_dir.get_child(&name).unwrap();
            continue;
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir"){
            cur_dir.add_child(Directory::new(line[4..].to_string()));
            continue;
        } else {
            let (size, name) = line.rsplit_once(" ").unwrap();
            let size = size.parse::<i32>().unwrap();
            let file = File::new(name.to_string(), size);
            cur_dir.add_file(file);
        }
    }

    return 0;
}