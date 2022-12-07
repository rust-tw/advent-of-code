use std::collections::HashMap;

pub fn challenge_07(lines: Vec<&str>) -> (usize, usize) {
    let mut root = FsNode::create_dir();
    let mut cur_path = Vec::new();
    let mut cur_dir = &mut root;
    for (idx, line) in lines.into_iter().enumerate() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        if tokens[0] == "$" {
            match tokens[1] {
                "ls" => continue,
                "cd" => {
                    match tokens[2] {
                        ".." => {
                            cur_path.pop();
                        }
                        "/" => cur_path.clear(),
                        dir_name => cur_path.push(dir_name),
                    };

                    // `cur_path` changed. Now update `cur_dir`.
                    cur_dir = cur_path
                        .iter()
                        .fold(&mut root, |dir, &d| {
                            dir.get_contents_mut()
                                .expect(&format!("Line {}: non-dir found in path.", idx + 1))
                                .get_mut(d)
                                .expect(&format!("Line {}: {} is not a dir.", idx + 1, d))
                        });
                }
                _ => panic!("Line {}: Invalid command `{}`", idx + 1, tokens[1]),
            }
        } else {
            if tokens[0] == "dir" {
                cur_dir
                    .get_contents_mut()
                    .unwrap()
                    .insert(String::from(tokens[1]), FsNode::create_dir());
            } else {
                cur_dir.get_contents_mut().unwrap().insert(
                    tokens[1].to_owned(),
                    FsNode::create_file(tokens[0].parse().unwrap()),
                );
            }
        }
    }

    root.eval_size();

    let target = 30000000 - (70000000 - root.get_size());

    (root.part_1(), root.part_2(target, 70000000))
}

#[derive(Debug)]
enum FsNode {
    File {
        size: usize,
    },
    Dir {
        contents: HashMap<String, FsNode>,
        size: usize,
    },
}

impl<'a> FsNode {
    fn create_file(size: usize) -> Self {
        Self::File { size }
    }

    fn create_dir() -> Self {
        Self::Dir {
            contents: HashMap::new(),
            size: 0,
        }
    }

    fn get_size(&self) -> usize {
        match self {
            Self::File { size, .. } => *size,
            Self::Dir { size, .. } => *size,
        }
    }

    fn get_contents_mut(&mut self) -> Option<&mut HashMap<String, FsNode>> {
        match self {
            Self::Dir { contents, .. } => Some(contents),
            _ => None,
        }
    }

    fn eval_size(&mut self) {
        if let Self::Dir { contents, size, .. } = self {
            *size = contents
                .iter_mut()
                .map(|(_, f)| {
                    f.eval_size();
                    f.get_size()
                })
                .sum();
        }
    }

    fn part_1(&self) -> usize {
        match self {
            Self::Dir { contents, size } => {
                let this_size = if *size < 100000 { *size } else { 0 };
                this_size + contents.iter().map(|(_, f)| f.part_1()).sum::<usize>()
            }
            _ => 0,
        }
    }

    fn part_2(&self, target: usize, candidate: usize) -> usize {
        match self {
            Self::Dir { contents, size } => {
                let best_of_children = contents
                    .iter()
                    .fold(candidate, |accu, (_, f)| f.part_2(target, accu));

                // Check if size of this node is better than children's.
                if *size > target && *size < best_of_children {
                    *size
                } else {
                    best_of_children
                }
            }
            _ => candidate,
        }
    }
}
