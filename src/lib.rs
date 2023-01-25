use std::collections::BTreeSet;
use std::rc::Rc;

pub struct Dag<T: Ord> {
    roots: BTreeSet<Rc<Node<T>>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<T: Ord> {
    elem: T,
    children: BTreeSet<Rc<Node<T>>>,
}

impl<T: Ord> Dag<T> {
    pub fn new() -> Self {
        Dag {
            roots: BTreeSet::new(),
        }
    }

    pub fn add_node(&mut self, elem: T) -> Rc<Node<T>> {
        let node = Rc::new(Node {
            elem,
            children: BTreeSet::new(),
        });
        self.roots.insert(node.clone());
        node
    }
}

impl<T: Ord> Default for Dag<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
