#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(special_module_name)]

use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug)]
pub struct CDLNode<T> {
    pub value: T,
    // aka node pointers, but without borrow checking
    pub next: usize,
    pub prev: usize,
    // aka you have reference to self, but without borrow checking
    pub index: usize,
}

#[derive(Debug)]
pub struct CDL<T> {
    nodes: RefCell<Vec<CDLNode<T>>>,
}

impl<T> CDL<T> {
    pub fn new() -> CDL<T> {
        return CDL {
            nodes: RefCell::new(vec![]),
        };
    }

    pub fn add(&mut self, value: T) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        let index = nodes.len();

        if nodes.is_empty() {
            // First node - points to itself
            let node = CDLNode {
                value,
                next: index, // Points to self
                prev: index, // Points to self
                index,
            };
            nodes.push(node);
        } else {
            // Add to existing circular list
            let head_idx = 0;
            let tail_idx = nodes[head_idx].prev;

            let node = CDLNode {
                value,
                next: head_idx,
                prev: tail_idx,
                index,
            };

            nodes.push(node);
            // Update links
            nodes[head_idx].prev = index;
            nodes[tail_idx].next = index;
        }

        index
    }

    pub fn add_after_idx(&self, after_index: usize, value: T) -> Option<usize> {
        if after_index >= self.nodes.borrow().len() {
            return None; // Invalid index
        }

        let mut nodes = self.nodes.borrow_mut();

        let new_index = nodes.len();
        let next_index = nodes[after_index].next;

        // Create new node
        let new_node = CDLNode {
            value,
            next: next_index,
            prev: after_index,
            index: new_index,
        };

        nodes.push(new_node);

        // Update links
        nodes[after_index].next = new_index;
        nodes[next_index].prev = new_index;

        Some(new_index)
    }

    pub fn add_after_node(&self, after_node: &CDLNode<T>, value: T) -> usize {
        let after_index = after_node.index; // Assuming nodes know their index
        let mut nodes = self.nodes.borrow_mut();

        let new_index = nodes.len();
        let next_index = nodes[after_index].next;

        let new_node = CDLNode {
            value,
            next: next_index,
            prev: after_index,
            index: new_index,
        };

        nodes.push(new_node);
        nodes[after_index].next = new_index;
        nodes[next_index].prev = new_index;

        new_index
    }

    pub fn get_node(&self, index: usize) -> Option<Ref<'_, CDLNode<T>>> {
        let nodes = self.nodes.borrow();
        if index < nodes.len() {
            Some(Ref::map(nodes, |n| &n[index]))
        } else {
            None
        }
    }

    pub fn get_node_mut(&self, index: usize) -> Option<RefMut<'_, CDLNode<T>>> {
        let nodes = self.nodes.borrow_mut();
        if index < nodes.len() {
            Some(RefMut::map(nodes, |n| &mut n[index]))
        } else {
            None
        }
    }
}

fn main() {
    let mut g: CDL<i32> = CDL::new();
    let (idx_1, idx_2, idx_3, idx_5, idx_8) = (g.add(1), g.add(2), g.add(3), g.add(5), g.add(8));

    {
        let mut node_3 = g.get_node_mut(idx_3).unwrap();
        node_3.value *= 10;
    }

    // Get the node reference and drop it before calling add_after_node
    let node_ref = g.get_node(idx_3).unwrap();
    let node_copy = *node_ref; // If CDLNode implements Copy
    drop(node_ref); // Explicitly drop the borrow

    g.add_after_node(&*g.get_node(idx_3).unwrap(), 100); //  <-- happens here

    // println!("{:#?}", node_3);

    println!("{:#?}", g);
}
