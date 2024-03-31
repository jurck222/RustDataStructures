use crate::{binary_search_tree::BinarySearchTree, hash_table::HashTable, linked_list::LinkedList};

mod binary_search_tree;
mod hash_table;
mod linked_list;

fn main() {
    run_linked_list();
    run_binary_search_tree();
    run_hash_table();
}

fn run_linked_list() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Pop {}", list.pop().unwrap());
    list.push(4);
    println!("Pop {}", list.pop().unwrap());
    println!("length = {}", list.length());
}

fn run_binary_search_tree() {
    let mut bst = BinarySearchTree::new();
    bst.insert(2);
    bst.insert(1);
    bst.insert(3);
    bst.insert(10);
    bst.insert(5);
    bst.insert(2);
    bst.in_order(&bst.root);
    println!("find 2 : {}", bst.find(2));
    println!("find 5 : {}", bst.find(5));
    println!("find 15 : {}", bst.find(15))
}

fn run_hash_table() {
    let mut hash = HashTable::new();
    hash.insert(String::from("Janez"), 1_000_000);
    hash.insert(String::from("Marko"), 2_000_000);
    hash.insert(String::from("Joza"), 1_500_000);
    let x: String = String::from("Marko");
    println!("value for {} = {}", x, hash.get(&x).unwrap());
}
