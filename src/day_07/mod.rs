#[path = "../utils/mod.rs"] mod utils;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Directory<'a> {
    // We need to hold a recersive mutable vector - the internet tells me this is a bad idea :(
    // Nevertheless use std::cell::RefCell for reference management
    // Use std::rc for SmartPointer functionality
    // <'a> controls the lifetime of the object preventing dangling pointers
    directory_ref: Rc<RefCell<DirectoryContainer<'a>>>,
}

struct DirectoryContainer<'a> {
    // This string split controls the lifetime of the objects
    name: &'a str,
    files: Vec<File>,
    // Strong pointer to children, we own their memory
    directories: Vec<Rc<RefCell<DirectoryContainer<'a>>>>,
    // Weak pointer to parent, we do not own their memory
    parent: Option<Weak<RefCell<DirectoryContainer<'a>>>>,
}

struct File {
    #[allow(dead_code)]
    name: String,
    size: u64,
}

impl<'a> Directory<'a> {
    fn get_sizes(
        &self,
        directory_ref: Rc<RefCell<DirectoryContainer<'a>>>,
        sizes: &mut Vec<u64>,
    ) -> u64 {
        let mut total_size = 0;
        total_size += directory_ref.borrow().files.iter().map(|f| f.size).sum::<u64>();
        for sub_directory in directory_ref.borrow().directories.iter() {
            total_size += self.get_sizes(sub_directory.clone(), sizes);
        }
        sizes.push(total_size);
        return total_size;
    }
}

fn parse_lines(lines: &Vec<String>) -> Directory {
    let directory_structure = Directory {
        directory_ref: Rc::new(RefCell::new(DirectoryContainer {
            parent: None,
            name: "/",
            files: vec![],
            directories: vec![],
        })),
    };

    // Strong pointer to current directory i.e. "/"
    let mut current_directory = directory_structure.directory_ref.clone();

    // Parse lines
    for line in lines {
        let split = line.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        if split[0] == "$" {
            if split[1] == "cd" {
                let name = &split[2];
                if name == "/" {
                    // Only called in line 1, we can skip this as root already created
                    continue;
                } else if name == ".." {
                    // Copy weak pointer to parent
                    // Upgrade this to get the strong pointer and set as current directory
                    let weak_parent_pointer = current_directory.borrow().parent.as_ref().unwrap().clone();
                    current_directory = weak_parent_pointer.upgrade().unwrap();
                } else {
                    // Copy strong pointer to child and set as current directory
                    let strong_child_pointer = current_directory.borrow().directories.iter().find(|n| n.borrow().name == name).unwrap().clone();
                    current_directory = strong_child_pointer;
                }
            } else {
                // Function is "ls" we dont need to do anything here
                // Files will get processed in the else block for split[0]
                continue;
            }
        } else if split[0] == "dir" {
            let sub_directory_name = &line[4..];
            // Add new empty directory to sub directories list
            // Set parent as weak pointer to current diectory
            current_directory.borrow_mut().directories.push(Rc::new(RefCell::new(DirectoryContainer {
                name: sub_directory_name,
                files: vec![],
                parent: Some(Rc::downgrade(&current_directory)),
                directories: vec![],
                })));
        } else {
            // Add new file to files list
            let file = line.split(' ').collect::<Vec<&str>>();
            current_directory.borrow_mut().files.push(File {
                name: String::from(file[1]),
                size: file[0].parse::<u64>().unwrap(),
            });
        }
    }
    return directory_structure;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_07/input.txt").expect("Failed to read line from file");
    let directory_structure = parse_lines(&lines);
    let mut directory_sizes = vec![];
    let required_space = 30000000 - (70000000 - directory_structure.get_sizes(directory_structure.directory_ref.clone(), &mut directory_sizes));
    let size_less_than_100000 = directory_sizes.clone().into_iter().filter(|&s| s <= 100000).sum::<u64>();
    let file_to_remove_size = directory_sizes.into_iter().filter(|&s| s >= required_space).min().unwrap();
    assert_eq!(size_less_than_100000, 1367870);
    assert_eq!(file_to_remove_size, 549173);
}