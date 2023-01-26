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

    fn add_child(&self, child: Rc<Link<T>>) {
        self.0.borrow_mut().children.insert(child.clone());
    }

    fn remove_child(&self, child: Rc<Link<T>>) {
        self.0.borrow_mut().children.remove(&child);
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

    pub fn add_edge(&mut self, parent: Rc<Link<T>>, child: Rc<Link<T>>) {
        parent.as_ref().add_child(child.clone());

        self.roots.remove(&child);
    }

    // NOTE: this doesn't add to roots when a node no longer has parents,
    // only for use in transitive reduce.
    fn remove_edge(&mut self, parent: Rc<Link<T>>, child: Rc<Link<T>>) {
        parent.as_ref().remove_child(child);
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
