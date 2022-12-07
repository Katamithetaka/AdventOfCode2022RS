use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub fn make_path(dir: String, name: &str) -> String {
    if dir.ends_with("/") {
        return dir + name;
    } else {
        return dir + "/" + name;
    }
}

#[derive(Debug)]
pub struct File {
    size: u64,
    full_path: String,
    file_name: String,
}
#[allow(dead_code)]

impl File {
    pub fn print(&self, tab: String) {
        println!("{}- {} (file, size={})", tab, self.file_name, self.size);
    }
}

#[derive(Debug)]
pub struct Directory {
    full_path: String,
    file_name: String,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}
#[allow(dead_code)]

impl Directory {
    pub fn print_tree(&self, tab: String) {
        println!("{}- {} (dir)", tab, self.file_name);
        let mut nodes = self.nodes.values().collect::<Vec<_>>();
        nodes.sort_by(|a, b| a.borrow().partial_cmp(&b.borrow()).unwrap());
        for node in nodes.iter() {
            node.borrow().print_tree(tab.clone() + "  ");
        }
    }

    pub fn get_directories(&self, result: &mut Vec<Rc<RefCell<Node>>>) {
        self.nodes.values().for_each(|f| {
            let cloned = Rc::clone(&f);
            let dir_node = Ref::filter_map(cloned.borrow(), |enum_option| match enum_option {
                Node::DirectoryNode(dir) => Some(dir),
                _ => None,
            });

            let _ = dir_node.map(|dir| {
                result.push(Rc::clone(&f));
                dir.get_directories(result);
            });
        });
    }

    pub fn get_files(&self, result: &mut Vec<Rc<RefCell<Node>>>) {
        self.nodes.values().for_each(|f| {
            let cloned = Rc::clone(&f);
            let node = Ref::filter_map(cloned.borrow(), |enum_option| match enum_option {
                Node::FileNode(file) => Some(file),
                Node::DirectoryNode(_) => None,
            });

            let _ = node.map(|_| result.push(Rc::clone(&f)));
        });
    }

    pub fn get_full_path(&self) -> String {
        return self.full_path.clone();
    }

    pub fn get_size(&self) -> u64 {
        let mut result = 0;
        for (_, node) in self.nodes.iter() {
            result += node.borrow().get_size();
        }
        result
    }

    pub fn get_directory(&mut self, name: &str) -> Rc<RefCell<Node>> {
        let node = self.nodes.entry(String::from(name));
        let node = match node {
            std::collections::hash_map::Entry::Occupied(o) => Rc::clone(o.get()),
            std::collections::hash_map::Entry::Vacant(_) => {
                let node = Node::DirectoryNode(Directory {
                    full_path: make_path(self.full_path.clone(), name),
                    file_name: String::from(name),
                    nodes: HashMap::new(),
                });

                self.add_directory(node)
            }
        };

        return Rc::clone(&node);
    }

    pub fn add_directory(&mut self, node: Node) -> Rc<RefCell<Node>> {
        if let Node::DirectoryNode(dir) = node {
            let dir_name = dir.file_name.clone();
            let result = Rc::new(RefCell::new(Node::DirectoryNode(dir)));
            self.nodes.insert(dir_name, Rc::clone(&result));
            return Rc::clone(&result);
        } else {
            panic!("Tried to handle a file like it was a directory!");
        }
    }

    pub fn add_file(&mut self, node: Node) -> Rc<RefCell<Node>> {
        if let Node::FileNode(file) = node {
            let file_name = file.file_name.clone();
            let result = Rc::new(RefCell::new(Node::FileNode(file)));
            self.nodes.insert(file_name, Rc::clone(&result));
            return Rc::clone(&result);
        } else {
            panic!("Tried to handle a file like it was a directory!");
        }
    }
}

#[derive(Debug)]
pub enum Node {
    FileNode(File),
    DirectoryNode(Directory),
}

impl Node {
    pub fn print_tree(&self, tab: String) {
        match self {
            Node::FileNode(file) => file.print(tab),
            Node::DirectoryNode(dir) => dir.print_tree(tab),
        }
    }

    pub fn get_full_path(&self) -> String {
        match self {
            Node::FileNode(file) => file.full_path.clone(),
            Node::DirectoryNode(dir) => dir.full_path.clone(),
        }
    }

    pub fn get_files(&self) -> Option<Vec<Rc<RefCell<Node>>>> {
        match self {
            Node::FileNode(_) => None,
            Node::DirectoryNode(dir) => Some({
                let mut result = vec![];
                dir.get_files(&mut result);
                result
            }),
        }
    }

    pub fn get_size(&self) -> u64 {
        match self {
            Node::FileNode(file) => file.size,
            Node::DirectoryNode(dir) => dir.get_size(),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::FileNode(l0), Self::FileNode(r0)) => l0.full_path.partial_cmp(&r0.full_path),
            (Self::DirectoryNode(l0), Self::DirectoryNode(r0)) => {
                l0.full_path.partial_cmp(&r0.full_path)
            }
            (Self::DirectoryNode(l0), Self::FileNode(r0)) => {
                l0.full_path.partial_cmp(&r0.full_path)
            }
            (Self::FileNode(l0), Self::DirectoryNode(r0)) => {
                l0.full_path.partial_cmp(&r0.full_path)
            }
        }
    }
}
#[allow(unused_variables)]

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FileNode(l0), Self::FileNode(r0)) => l0.full_path == r0.full_path,
            (Self::DirectoryNode(l0), Self::DirectoryNode(r0)) => l0.full_path == r0.full_path,
            (Self::DirectoryNode(l0), Self::FileNode(r0)) => false,
            (Self::FileNode(l0), Self::DirectoryNode(r0)) => false,
        }
    }
}

#[derive(Debug)]
pub struct FileExplorer {
    node: Rc<RefCell<Node>>,
    current_directory: String,
}

#[allow(dead_code)]
impl FileExplorer {
    pub fn new() -> Self {
        Self {
            node: Rc::new(RefCell::new(Node::DirectoryNode(Directory {
                full_path: String::from("/"),
                file_name: String::from("/"),
                nodes: HashMap::new(),
            }))),
            current_directory: String::from("/"),
        }
    }

    pub fn make_path(&self, file: &str) -> String {
        return make_path(self.current_directory.clone(), file);
    }

    pub fn get_directory_node(&self) -> Rc<RefCell<Node>> {
        if self.current_directory == "/" {
            return Rc::clone(&self.node);
        }

        let mut next = Rc::clone(&self.node);

        self.current_directory.split("/").skip(1).for_each(|f| {
            let temp_next = Rc::clone(&next);
            let mut dir = RefMut::map(Rc::as_ref(&temp_next).borrow_mut(), |f| match f {
                Node::DirectoryNode(node) => node,
                _ => panic!("Expected Directory, found File"),
            });

            next = dir.get_directory(f);
        });

        return next;
    }

    pub fn list_directory(&mut self) {
        return;
    }

    pub fn previous_directory(&mut self) {
        if self.current_directory == "/" {
            panic!("Tried to go further back than /");
        }

        if self.current_directory.ends_with("/") {
            self.current_directory.pop();
        }

        match self.current_directory.clone().rsplit_once("/") {
            Some((dir, _)) => self.current_directory = String::from(dir),
            None => {
                panic!("cd .. failed!{}", self.current_directory.len())
            }
        }
    }

    pub fn set_current_directory(&mut self, directory: &str) {
        self.current_directory = String::from(directory);
    }

    pub fn get_root_size(&self) -> u64 {
        return self.node.borrow().get_size();
    }

    pub fn next_directory(&mut self, directory: &str) {
        if !self.current_directory.ends_with('/') {
            self.current_directory.push('/');
        }
        self.current_directory.push_str(directory);
    }

    pub fn change_directory(&mut self, directory: &str) {
        if directory.starts_with("/") {
            self.set_current_directory(directory);
            return;
        }

        if directory == ".." {
            self.previous_directory();
            return;
        }

        self.next_directory(directory);
    }

    pub fn add_directory(&mut self, line: &str) {
        let (_, dir_name) = line.split_at(4);

        let node = Node::DirectoryNode(Directory {
            full_path: self.make_path(dir_name),
            file_name: String::from(dir_name),
            nodes: HashMap::new(),
        });

        let directory_node = self.get_directory_node();

        let directory_node = Rc::as_ref(&directory_node).borrow_mut();
        let mut directory = RefMut::map(directory_node, |f| match f {
            Node::DirectoryNode(directory) => directory,
            _ => panic!("Expected directory, got file instead."),
        });

        directory.add_directory(node);
    }

    pub fn add_file(&mut self, line: &str) {
        let (size, name) = line.split_once(" ").unwrap();

        let node = Node::FileNode(File {
            size: size.parse().unwrap(),
            full_path: self.make_path(name),
            file_name: String::from(name),
        });

        let directory_node = self.get_directory_node();

        let directory_node = Rc::as_ref(&directory_node).borrow_mut();
        let mut directory = RefMut::map(directory_node, |f| match f {
            Node::DirectoryNode(directory) => directory,
            _ => panic!("Expected directory, got file instead."),
        });

        directory.add_file(node);
    }

    pub fn handle_command(&mut self, line: &str) {
        let command = line.split_once(" ").unwrap().1;
        if command.starts_with("ls") {
            self.list_directory();
            return;
        }

        let (command, argument) = command.split_once(" ").unwrap();

        if command.trim() == "cd" {
            self.change_directory(argument);
        } else {
            panic!("Unhandled command! {}", command);
        }
    }

    pub fn handle_input(&mut self, line: &str) {
        if line.starts_with("$") {
            self.handle_command(line);
        } else if line.starts_with("dir") {
            self.add_directory(line);
        } else {
            self.add_file(line);
        }
    }

    pub fn get_directories(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut result = vec![];

        let _ = Ref::map(self.node.borrow(), |f| match f {
            Node::FileNode(_) => panic!("root dir cannot be a file!"),
            Node::DirectoryNode(dir) => {
                dir.get_directories(&mut result);
                dir
            }
        });

        result.push(Rc::clone(&self.node));

        return result;
    }

    pub fn get_files(&self) -> Vec<Rc<RefCell<Node>>> {
        let mut result = vec![];

        let _ = Ref::map(self.node.borrow(), |f| match f {
            Node::FileNode(_) => panic!("root dir cannot be a file!"),
            Node::DirectoryNode(dir) => {
                dir.get_files(&mut result);
                dir
            }
        });

        result.push(Rc::clone(&self.node));

        return result;
    }

    pub fn print_tree(&self) {
        self.node.borrow().print_tree(String::new());
    }
}
