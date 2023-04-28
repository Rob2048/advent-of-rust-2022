fn print_spaces(count: i32) {
    for _ in 0..count {
        print!(" ");
    }
}

type FileSystemNodeIndex = i32;

#[derive(Debug)]
enum FileSystemEntry {
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
struct Dir {
    index: i32,
    name: String,
    size: i32,
    parent: Option<FileSystemNodeIndex>,
    children: Vec<FileSystemNodeIndex>,
}

#[derive(Debug)]
struct File {
    index: i32,
    name: String,
    size: i32,
    parent: Option<FileSystemNodeIndex>,
}

#[derive(Debug)]
struct FileSystem {
    entries: Vec<FileSystemEntry>,
    root_node: Option<FileSystemNodeIndex>,
    current_node: Option<FileSystemNodeIndex>,
}

impl Dir {
    fn print(&self, file_system: &FileSystem, spacing: i32) {
        print_spaces(spacing);
        println!("- {} (dir, size={})", self.name, self.size);

        let new_spaces = spacing + 2;

        for child in &self.children {
            let node = file_system.get_node_imm(*child);

            match node {
                FileSystemEntry::Dir(dir) => dir.print(file_system, new_spaces),
                FileSystemEntry::File(file) => file.print(new_spaces),
            }
        }
    }
}

impl File {
    fn print(&self, spacing: i32) {
        print_spaces(spacing);
        println!("- {} (file, size={})", self.name, self.size);
    }
}

impl FileSystem {
    fn get_node(&mut self, index: FileSystemNodeIndex) -> &mut FileSystemEntry {
        return &mut self.entries[index as usize];
    }

    fn get_node_imm(&self, index: FileSystemNodeIndex) -> &FileSystemEntry {
        return &self.entries[index as usize];
    }

    fn add_dir(&mut self, name: &str) -> FileSystemNodeIndex {
        // Check that directory does not already exist.
        if self.current_node.is_some() {
            let current_dir = self.get_node_imm(self.current_node.unwrap());

            if let FileSystemEntry::Dir(s) = current_dir {
                for entry in &s.children {
                    if let FileSystemEntry::Dir(r) = self.get_node_imm(*entry) {
                        if r.name == name {
                            return r.index;
                        }
                    }
                }
            }
        }

        let node_index = (self.entries.len()) as FileSystemNodeIndex;

        let new_dir = FileSystemEntry::Dir(Dir {
            index: node_index,
            name: name.to_string(),
            size: 0,
            parent: self.current_node,
            children: Vec::new(),
        });
        self.entries.push(new_dir);

        match self.current_node {
            None => (),
            Some(node) => {
                if let FileSystemEntry::Dir(s) = self.get_node(node) {
                    s.children.push(node_index);
                } else {
                    panic!("Can not add a dir to a file");
                }
            }
        }

        return node_index;
    }

    fn add_file(&mut self, name: &str, size: i32) -> FileSystemNodeIndex {
        self.current_node
            .expect("There must be a current node to add a file to");

        let node_index = (self.entries.len()) as FileSystemNodeIndex;

        let new_file = FileSystemEntry::File(File {
            index: node_index,
            name: name.to_string(),
            size,
            parent: self.current_node,
        });
        self.entries.push(new_file);

        if let FileSystemEntry::Dir(s) = self.get_node(self.current_node.unwrap()) {
            s.children.push(node_index);
        } else {
            panic!("Can not add a file to a file");
        }

        return node_index;
    }

    fn move_up(&mut self) {
        self.current_node
            .expect("There must be a current node to move up");

        if let FileSystemEntry::Dir(s) = self.get_node(self.current_node.unwrap()) {
            self.current_node = s.parent;
        } else {
            panic!("Can not move up on a file");
        }
    }

    fn print(&self) {
        if self.root_node.is_none() {
            println!("No root node!");
            return;
        }

        let node = self.get_node_imm(self.root_node.unwrap());

        if let FileSystemEntry::Dir(s) = node {
            s.print(self, 0);
        } else {
            panic!("Root node must be a dir!");
        }
    }

    fn calculate_size(&mut self) {
        if self.root_node.is_none() {
            println!("No root node!");
            return;
        }

        type Visited = bool;

        let mut node_queue: Vec<(FileSystemNodeIndex, Visited)> = Vec::new();
        node_queue.push((self.root_node.unwrap(), false));

        loop {
            if node_queue.is_empty() {
                break;
            }

            let check_node_index = node_queue.len() - 1;
            let (node_index, visited) = node_queue[check_node_index];

            if !visited {
                let node = self.get_node_imm(node_index);

                if let FileSystemEntry::Dir(s) = node {
                    for child_node_index in &s.children {
                        let child_node = self.get_node_imm(*child_node_index);

                        match child_node {
                            FileSystemEntry::Dir(dir) => node_queue.push((dir.index, false)),
                            _ => (),
                        }
                    }
                } else {
                    panic!("Must be a directory");
                }

                let (_, visited) = &mut node_queue[check_node_index];
                *visited = true;
            } else {
                let node = self.get_node_imm(node_index);

                let mut local_size: i32 = 0;

                if let FileSystemEntry::Dir(s) = node {
                    for child_node_index in &s.children {
                        let child_node = self.get_node_imm(*child_node_index);

                        match child_node {
                            FileSystemEntry::File(file) => local_size += file.size,
                            FileSystemEntry::Dir(dir) => local_size += dir.size,
                        }
                    }
                } else {
                    panic!("Must be a directory");
                }

                let node = self.get_node(node_index);
                if let FileSystemEntry::Dir(s) = node {
                    s.size = local_size;
                }

                node_queue.pop();
            }
        }
    }

    fn aoc_part2(&self) {
        let total_size: i32 = if let FileSystemEntry::Dir(dir) = &self.entries[0] {
            dir.size
        } else {
            0
        };

        let target_free_space: i32 = 30000000;
        let free_space = 70000000 - total_size;
        let required_to_free = target_free_space - free_space;

        println!("Total size: {} Free space: {} Need to free: {}", total_size, free_space, required_to_free);

        let mut smallest_size: Option<i32> = None;
                
        for entry in &self.entries {
            if let FileSystemEntry::Dir(dir) = entry {
                if dir.size >= required_to_free {
                    if smallest_size.is_none() {
                        smallest_size = Some(dir.size)
                    } else if dir.size < smallest_size.unwrap() {
                        smallest_size = Some(dir.size);
                    }
                }
            }
        }

        println!("Free dir with size {}", smallest_size.unwrap());

    }
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_itr = file_data.lines();

    let mut file_system = FileSystem {
        entries: Vec::new(),
        root_node: None,
        current_node: None,
    };

    for line in line_itr {
        println!("    {line}");

        if line.starts_with('$') {
            println!("Found command");

            let mut param_itr = line.split(' ');
            param_itr.next().unwrap();

            let cmd = param_itr.next().unwrap();

            if cmd == "cd" {
                println!("CD");

                let dir = param_itr.next().unwrap();

                if dir == "/" {
                    let root_index = file_system.add_dir("/");
                    file_system.current_node = Some(root_index);
                    file_system.root_node = Some(root_index);
                } else if dir == ".." {
                    file_system.move_up();
                } else {
                    println!("Create dir {dir}");
                    let root_index = file_system.add_dir(dir);
                    file_system.current_node = Some(root_index);
                }
            } else if cmd == "ls" {
                println!("List");
            } else {
                unreachable!();
            }
        } else {
            let mut param_itr = line.split(' ');

            let param0 = param_itr.next().unwrap();
            let name = param_itr.next().unwrap();

            if param0 == "dir" {
                println!("list dir: {} ", name);
                file_system.add_dir(name);
            } else {
                let size: i32 = param0.parse().unwrap();
                println!("list file: {} {}", name, size);
                file_system.add_file(name, size);
            }
        }
    }

    file_system.print();
    file_system.calculate_size();
    println!("After calculating sizes:");
    file_system.print();
    file_system.aoc_part2();
}
