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
    let root = Aoc::default().root;
    let mut cwd = Rc::clone(&root);
    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {},
            ("$", "cd") => {
                match words[2] {
                    "/" => cwd = Rc::clone(&root),
                    ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                    dir_name => {
                        let new_dir = cwd.subdir.borrow().get(dir_name).unwrap().clone();
                        cwd = Rc::clone(&new_dir)
                    },
                }
            },
            ("dir", dir_name) => {
                cwd.subdir.borrow_mut().insert(dir_name.to_string(), Rc::new(Dir {
                    name:dir_name.to_string(),
                    size: RefCell::new(0),
                    parent: Some(Rc::clone(&cwd)),
                    subdir: RefCell::new(HashMap::new()),
                }));
            },
            (size, _name)=> *cwd.size.borrow_mut() += size.parse::<usize>().unwrap()
        }
    }

    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;
    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        let size  = dir.get_size();
        if size<=100000 {
            total += size
        }
    }
    println!("part 1: {}", total);

    let total_size = root.get_size();
    let free_space  = 70000000-total_size;
    let size_needed = 30000000-free_space;

    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = usize::MAX;
    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(Rc::clone(d))
        }

        let size = dir.get_size();
        if size >= size_needed {
            best = best.min(size)
        }
    }

    println!("part 2: {}", best)

}
#[derive(Default, Debug)]
struct Dir {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow() + self.subdir.borrow().values().fold(0, |a, b| a + b.get_size())
    }
}

#[derive(Default, Debug)]
struct Aoc {
    root: Rc<Dir>
}