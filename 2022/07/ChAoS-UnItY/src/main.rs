use std::cell::Cell;

#[derive(Debug)]
struct FileSystem {
    current_dir: usize,
    nodes: Vec<Node>
}

impl FileSystem {
    fn new() -> Self {
        Self { 
            current_dir: 0, 
            nodes: vec![Node::new_dir_node("/")]
        }
    }

    fn cd(&mut self, name: &str) {
        self.current_dir = match name {
            "/" => {
                0
            }
            ".." => {
                if let Node::DirNode { name: _, size: _, parent_node, children: _ } = &self.nodes[self.current_dir] {
                    parent_node.unwrap()
                } else {
                    unreachable!()
                }
            }
            _ => {
                if let Node::DirNode { name: _, size: _, parent_node: _, children } = &self.nodes[self.current_dir] {
                    *children.iter()
                        .filter(|&&node_index| self.nodes[node_index].name() == name)
                        .next()
                        .unwrap()
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn update_parent_dir_size(&mut self, size: u32, parent: usize) {
        match &self.nodes[parent] {
            Node::DirNode { name: _, size: dir_size, parent_node, children: _ } => {
                dir_size.set(dir_size.get() + size);

                if let Some(parent_node_index) = parent_node {
                    self.update_parent_dir_size(size, *parent_node_index);
                }
            },
            _ => {}
        }
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.add_node(Node::FileNode { 
            name: name.to_string(),
            size: Cell::new(size)
        });
        self.update_parent_dir_size(size, self.current_dir);
    }

    fn add_dir(&mut self, name: &str) {
        self.add_node(Node::DirNode { 
            name: name.to_string(),
            size: Cell::new(0),
            parent_node: Some(self.current_dir), 
            children: vec![]
        });
    }

    fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        if let Node::DirNode { name: _, size: _, parent_node: _, children } = &mut self.nodes[self.current_dir] {
            children.push(index);
        }
        index
    }

    fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().filter(|&node| match node {
            Node::DirNode { name: _, size: _, parent_node: _, children: _ } => true,
            _ => false
        })
    }

    fn size(&self) -> u32 {
        self.nodes[0].size()
    }
}

#[derive(Debug)]
enum Node {
    FileNode { name: String, size: Cell<u32> },
    DirNode { name: String, size: Cell<u32>, parent_node: Option<usize>, children: Vec<usize> },
}

impl Node {
    fn new_dir_node(name: &str) -> Node {
        Node::DirNode{
            name: name.to_string(), 
            size: Cell::new(0),
            parent_node: None,
            children: vec![]
        }
    }

    fn name(&self) -> String {
        match self {
            Node::FileNode { name, size: _ } => name.clone(),
            Node::DirNode { name, size: _, parent_node: _, children: _ } => name.clone(),
        }
    }

    fn size(&self) -> u32 {
        match self {
            Node::FileNode { name: _, size } => size.get(),
            Node::DirNode { name: _, size, parent_node: _, children: _ } => size.get(),
        }
    }
}

fn main() {
    let data = include_str!("../Day07.txt");
    let fs = process_data(data);

    println!("{:?}", part1(&fs));
    println!("{:?}", part2(&fs));
}

fn part1(fs: &FileSystem) -> u32 {
    fs.iter()
        .map(|node| node.size())
        .filter(|&size| size < 100_000)
        .sum()
}

fn part2(fs: &FileSystem) -> u32 {
    let mut size_of_dirs = fs.iter()
        .map(|node| node.size())
        .collect::<Vec<_>>();

    size_of_dirs.sort();

    *size_of_dirs.iter()
        .find(|&&size| size >= (30_000_000 - (70_000_000 - fs.size())))
        .unwrap()
}

fn process_data(data: &'static str) -> FileSystem {
    let mut file_sys = FileSystem::new();

    for mut executions in data
        .replace("\r\n", "\n")
        .split("$")
        .filter(|s| !s.is_empty())
        .map(|line| line.split("\n"))
    {
        let command = executions.next().unwrap();

        match &command[1..=2] {
            "cd" => file_sys.cd(&command[4..]),
            "ls" => {
                let mut executions_vec = executions.collect::<Vec<_>>();

                for file in executions_vec.drain(..executions_vec.len() - 1) {
                    let (attribute, name) = file.split_once(" ").unwrap();

                    if attribute.starts_with("dir") {
                        file_sys.add_dir(name);
                    } else {
                        file_sys.add_file(name, attribute.parse::<u32>().unwrap());
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    file_sys
}
