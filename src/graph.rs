use typed_arena::Arena;
use std::{cell::UnsafeCell, collections::HashSet, hash::Hash};

pub struct Node<'a, T> {
    pub data: T,
    pub edges: UnsafeCell<Vec<&'a Node<'a, T>>>,
}

impl<'n, T> Node<'n, T>
    where T: PartialEq + Eq + Hash + Clone,
{
    pub fn new<'arena: 'n>(datum: T, arena: &'arena Arena<Node<'arena, T>>) -> &'arena Self {
        arena.alloc(Self {
            data: datum,
            edges: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn traverse<F: Fn(&T)>(&self, f: &F, seen: &mut HashSet<T>) {
        if seen.contains(&self.data) {
            return;
        }

        f(&self.data);
        seen.insert(self.data.clone());

        unsafe {
            for n in &*self.edges.get() {
                n.traverse(f, seen);
            }
        }
    }

    pub fn first(&self) -> &Self {
        unsafe {
            (*self.edges.get())[0]
        }
    }
}
