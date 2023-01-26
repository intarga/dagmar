use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<T: Ord> {
    elem: T,
    children: BTreeSet<Rc<Link<T>>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Link<T: Ord>(RefCell<Node<T>>);

pub struct Dag<T: Ord> {
    roots: BTreeSet<Rc<Link<T>>>,
}

impl<T: Ord> Link<T> {
    fn new(elem: T) -> Self {
        Link(RefCell::new(Node {
            elem,
            children: BTreeSet::new(),
        }))
    }
}

impl<T: Ord> Dag<T> {
    pub fn new() -> Self {
        Dag {
            roots: BTreeSet::new(),
        }
    }

    pub fn add_node(&mut self, elem: T) -> Rc<Link<T>> {
        let link = Rc::new(Link::new(elem));
        self.roots.insert(link.clone());
        link
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
