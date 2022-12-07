use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let mut root: HashMap<&str, RandomState> = HashMap::new();
    let mut cwd = HashMap::new();
    let mut stack = vec![];

    for line in input.lines() {
        if line.starts_with("$") {
            if line.contains("cd") {
                let dir = line.split_whitespace().nth(2).unwrap();
                if dir == "/" {
                    cwd = root.clone();
                    stack = vec![];
                }
                else if dir == ".." {
                    cwd = stack.pop().unwrap();
                }
                else {
                    if !cwd.contains_key(dir) {
                        cwd.insert(dir, RandomState::new())
                    }
                    stack.push(cwd.clone());
                    cwd = HashMap::new();
                    cwd.insert(dir, RandomState::new())
                }
            }
        }
    }
}
