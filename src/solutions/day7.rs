use crate::utils::print_answer;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

struct Directory {
    sub_dirs: HashMap<String, Rc<RefCell<Directory>>>,
    parent_dir: Option<Rc<RefCell<Directory>>>,
    files: Vec<u32>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            sub_dirs: HashMap::new(),
            parent_dir: None,
            files: Vec::new(),
        }
    }

    fn new_with_parent(parent_dir: Rc<RefCell<Directory>>) -> Directory {
        Directory {
            sub_dirs: HashMap::new(),
            parent_dir: Some(parent_dir),
            files: Vec::new(),
        }
    }

    fn add_sub_dir(&mut self, dir_name: String, dir: Rc<RefCell<Directory>>) {
        self.sub_dirs.insert(dir_name, dir);
    }

    fn add_file(&mut self, size: u32) {
        self.files.push(size);
    }

    fn get_sub_dir(&self, dir_name: &str) -> Rc<RefCell<Directory>> {
        self.sub_dirs.get(dir_name).unwrap().clone()
    }

    fn get_parent_dir(&self) -> Rc<RefCell<Directory>> {
        self.parent_dir.as_ref().unwrap().clone()
    }

    fn size(&self) -> u32 {
        let mut size = 0;

        size += self.files.iter().sum::<u32>();
        size += self.sub_dirs.iter().fold(0, |acc, (_, dir)| acc + dir.borrow().size());

        return size;
    }

    fn get_max_size(&self, max_size: u32) -> Vec<u32> {
        let mut sizes = Vec::new();

        if self.size() < max_size {
            sizes.push(self.size());
        }

        for (_, dir) in self.sub_dirs.iter() {
            sizes.append(&mut dir.borrow().get_max_size(max_size));
        }

        sizes
    }

    fn get_min_size(&self, min_size: u32) -> Vec<u32> {
        let mut sizes = Vec::new();

        if self.size() > min_size {
            sizes.push(self.size());
        }

        for (_, dir) in self.sub_dirs.iter() {
            sizes.append(&mut dir.borrow().get_min_size(min_size));
        }

        sizes
    }
}

fn part1(input: &str) -> u32 {
    let root = Rc::new(RefCell::new(Directory::new()));
    let mut cur_dir = root.clone();

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line[..] {
            ["$", "ls"] => (),
            ["$", "cd", "/"] => {
                cur_dir = root.clone();
            },
            ["$", "cd", ".."] => {
                let dir = cur_dir.borrow().get_parent_dir();
                cur_dir = dir;
            },
            ["$", "cd", directory] => {
                let dir = cur_dir.borrow().get_sub_dir(directory);
                cur_dir = dir;
            },
            ["dir", dir_name] => {
                let new_dir = Rc::new(RefCell::new(
                    Directory::new_with_parent(cur_dir.clone()))
                );
                cur_dir.borrow_mut().add_sub_dir(dir_name.to_string(), new_dir);
            },
            [file_size, _file_name] => {
                let file_size = file_size.parse::<u32>().unwrap();
                cur_dir.borrow_mut().add_file(file_size);
            },
            _ => (),
        }
    }

    return root.borrow().get_max_size(100000).iter().sum();
}

fn part2(input: &str) -> u32 {
    let root = Rc::new(RefCell::new(Directory::new()));
    let mut cur_dir = root.clone();

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line[..] {
            ["$", "ls"] => (),
            ["$", "cd", "/"] => {
                cur_dir = root.clone();
            },
            ["$", "cd", ".."] => {
                let dir = cur_dir.borrow().get_parent_dir();
                cur_dir = dir;
            },
            ["$", "cd", directory] => {
                let dir = cur_dir.borrow().get_sub_dir(directory);
                cur_dir = dir;
            },
            ["dir", dir_name] => {
                let new_dir = Rc::new(RefCell::new(
                    Directory::new_with_parent(cur_dir.clone()))
                );
                cur_dir.borrow_mut().add_sub_dir(dir_name.to_string(), new_dir);
            },
            [file_size, _file_name] => {
                let file_size = file_size.parse::<u32>().unwrap();
                cur_dir.borrow_mut().add_file(file_size);
            },
            _ => (),
        }
    }

    const AVAILABLE_DISK_SPACE: u32 = 70000000;
    const REQUIRED_UNUSED_SPACE: u32 = 30000000;

    let current_unused_space: u32 = AVAILABLE_DISK_SPACE - root.borrow().size();
    let to_delete_size: u32 =  REQUIRED_UNUSED_SPACE - current_unused_space;

    let sizes: Vec<u32> = root.borrow().get_min_size(to_delete_size);

    return *sizes.iter().min().unwrap();
}