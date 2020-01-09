extern crate rand;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use rand::Rng;

#[derive(Debug)]
enum State {
    OCCUPIED,
    DELETED,
}

macro_rules! node {
   ($data: expr) => {
        Node::new($data)
    }
}

#[derive(Debug, Clone)]
struct Node {
    data: String
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Node {
    fn new(data: String) -> Self {
        Node {
            data
        }
    }
}

#[derive(Debug)]
struct HashTable {
    radio_expand: f64,
    ratio_shrink: f64,
    size: usize,
    count: usize,
    table: HashMap<usize, Option<Node>>,
    table_state: HashMap<usize, State>,
}


impl Default for HashTable {
    fn default() -> HashTable {
        Self {
            size: 1,
            count: 0,
            radio_expand: 0.75,
            ratio_shrink: 0.5,
            table: HashMap::new(),
            table_state: HashMap::new(),
        }
    }
}


impl HashTable {
    pub fn insert(&mut self, node: Node) {
        let index = self.get_table_index(&self.table, &node);
        HashTable::insert_table(&mut self.table, &mut self.table_state, index, &node);
        self.count += 1;
        self.table_doubling();
    }

    fn insert_table(table: &mut HashMap<usize, Option<Node>>, table_state: &mut HashMap<usize, State>, index: usize, node: &Node) {
        table.insert(index, Some(Node::new(node.data.clone())));
        table_state.insert(index, State::OCCUPIED);
    }

    fn rehash(&mut self, node: &Node, table: &mut HashMap<usize, Option<Node>>, table_state: &mut HashMap<usize, State>) {
        let index = self.get_table_index(&table, node);
        HashTable::insert_table(table, table_state, index, &node);
        table_state.insert(index, State::OCCUPIED);
    }

    fn get_load_factor(&self) -> f64 {
        self.count as f64 / self.size as f64
    }


    fn get_hash_value(&self, node: &Node) -> u64 {
        let mut s = DefaultHasher::new();
        node.hash(&mut s);
        s.finish()
    }

    fn get_table_index(&self, table: &HashMap<usize, Option<Node>>, node: &Node) -> usize {
        let hash = self.get_hash_value(node);
        let mut index = hash as usize % self.size;
        loop {
            match table.get(&index) {
                Some(_) => {
                    index += 1;
                    index %= self.size;
                }
                None => {
                    return index;
                }
            }
        }
    }

    fn preform_rehash(&mut self) {
        let mut table = HashMap::<usize, Option<Node>>::new();
        let mut table_state = HashMap::<usize, State>::new();
        self.table.clone().iter().for_each(|(_, v)| {
            match v {
                Some(n) => {
                    self.rehash(n, &mut table, &mut table_state);
                }
                None => {}
            }
        });
        self.table = table;
        self.table_state = table_state;
    }

    fn table_doubling(&mut self) {
        let load_factor = self.get_load_factor();
        if load_factor <= self.radio_expand {
            return;
        }
        self.size *= 2;
        self.preform_rehash();
    }

    fn table_shrinking(&mut self) {
        let load_factor = self.get_load_factor();
        if load_factor >= self.ratio_shrink {
            return;
        }
        self.size /= 2;
        self.preform_rehash();
    }

    fn delete(&mut self, node: Node) {
        let index = self.search(&node);
        match index {
            Some(i) => {
                self.table_state.insert(i, State::DELETED);
                self.table.remove(&i);
                self.count -= 1;
                self.table_shrinking();
            }
            None => {}
        }
    }

    fn search(&self, node: &Node) -> Option<usize> {
        let hash = self.get_hash_value(node);
        let index = hash as usize % self.size;
        let mut search_index = index;
        loop {
            let value = self.table.get(&search_index);
            match value {
                Some(n) => {
                    match n {
                        Some(search_node) => {
                            if search_node.data == node.data {
                                return Some(search_index);
                            } else {
                                search_index += 1;
                            }
                        }
                        None => {
                            match self.table_state.get(&search_index) {
                                Some(s) => {
                                    match *s {
                                        State::DELETED => {
                                            search_index += 1;
                                        }
                                        _ => {}
                                    }
                                }
                                None => {
                                    return None;
                                }
                            }
                        }
                    }
                    search_index %= self.size;
                    if search_index == index {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut hash_table: HashTable = HashTable::default();
    for _i in 0..=1000 {
        let r: i32 = rng.gen_range(0, 1000);
        let chance: f64 = rng.gen();
        let node_value = format!("{}", r);
        let node = node!(node_value);
        if chance >= 0.5 {
            hash_table.insert(node);
        } else {
            hash_table.delete(node);
        }
    }
    println!("{} {}", hash_table.count, hash_table.size);
}
