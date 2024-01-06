use std::collections::VecDeque;
use std::fmt::Display;

const ALPHABET_COUNT: usize = 26;
const NUMBERS_COUNT: usize = 9;
const TABLE_SIZE: usize = ALPHABET_COUNT + NUMBERS_COUNT;

pub fn solve(input: &str) -> u64 {
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let dict = Dictionary::create_with_words(&words);
    // println!("{dict}");

    input.lines().map(|s| dict.handle_line(s)).sum()
}

struct Node {
    pch: char,
    parent: Option<usize>,
    children: [Option<usize>; TABLE_SIZE],
    suffix_link: usize,
    value: Option<u64>,
}

impl Node {
    fn new() -> Self {
        Node {
            pch: '$',
            parent: None,
            children: [None; TABLE_SIZE],
            suffix_link: 0,
            value: None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .children
            .iter()
            .map(|n| match n {
                None => String::from("-1"),
                Some(n) => format!("{:2}", n),
            })
            .collect::<Vec<_>>()
            .join(",");

        let parent = match self.parent {
            None => String::from("-1"),
            Some(n) => n.to_string(),
        };

        let value = match self.value {
            None => String::from("-"),
            Some(n) => n.to_string(),
        };
        write!(
            f,
            "\'{}\', {value:2}, {parent:2}, [{s}], {}",
            self.pch, self.suffix_link
        )
    }
}

struct Dictionary {
    dict: Vec<Node>,
}

impl Dictionary {
    pub fn create_with_words(words: &[(&str, u64)]) -> Self {
        let mut dict = vec![Node::new()];
        for (word, value) in words {
            let mut index = 0;
            for ch in word.chars() {
                let ch_index = Self::index_from_alphanum(ch);
                index = match dict[index].children[ch_index] {
                    Some(n) => n,
                    None => {
                        let n = dict.len();
                        let mut node = Node::new();
                        node.parent = Some(index);
                        node.pch = ch;
                        dict.push(node);
                        dict[index].children[ch_index] = Some(n);
                        n
                    }
                }
            }
            dict[index].value = Some(*value);
        }

        let mut queue = VecDeque::new();
        queue.push_back(0);

        // construct the automaton
        while let Some(i) = queue.pop_front() {
            // pushes nodes in 'next' list into the queue (BFS)
            dict[i]
                .children
                .iter()
                .filter_map(|n| n.clone())
                .for_each(|n| queue.push_back(n));

            dict[i].suffix_link = match dict[i].parent {
                // for root & layer 1 nodes, just back to root
                None | Some(0) => 0,
                Some(parent) => {
                    let ch_index = Self::index_from_alphanum(dict[i].pch);
                    match dict[dict[parent].suffix_link].children[ch_index] {
                        Some(link_index) => link_index,
                        None => match dict[0].children[ch_index] {
                            Some(link_index) => link_index,
                            None => 0,
                        },
                    }
                }
            }
        }

        Dictionary { dict }
    }

    pub fn handle_line(&self, s: &str) -> u64 {
        let mut head = None;
        let mut tail = None;

        let mut index = 0;

        for c in s.chars() {
            // println!("{index}, {c}");
            let ch_index = Self::index_from_alphanum(c);
            if let Some(i) = self.dict[index].children[ch_index] {
                index = i;
            } else {
                loop {
                    index = self.dict[index].suffix_link;
                    if let Some(i) = self.dict[index].children[ch_index] {
                        index = i;
                        break;
                    } else if index == 0 {
                        break;
                    }
                }
            }

            if self.dict[index].value.is_some() {
                if head.is_none() {
                    head = self.dict[index].value;
                }
                tail = self.dict[index].value;
            }
        }

        // println!("{head},{tail}");
        match (head, tail) {
            (Some(h), Some(t)) => h * 10 + t,
            _ => 0,
        }
    }

    fn index_from_alphanum(c: char) -> usize {
        match c {
            '1'..='9' => (c as usize) - ('1' as usize) + ALPHABET_COUNT,
            'a'..='z' => (c as usize) - ('a' as usize),
            _ => 0,
        }
    }
}

impl Display for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .dict
            .iter()
            .enumerate()
            .map(|(idx, n)| format!("{:3}: {}", idx, n))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{s}")
    }
}
