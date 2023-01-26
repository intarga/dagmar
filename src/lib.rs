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
        self.0.borrow_mut().children.insert(child);
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
    fn remove_edge(parent: Rc<Link<T>>, child: Rc<Link<T>>) {
        parent.as_ref().remove_child(child);
    }

    fn recursive_parent_remove(parent: Rc<Link<T>>, child: Rc<Link<T>>) {
        Self::remove_edge(parent.clone(), child.clone());
        for granchild in child.0.borrow().children.iter() {
            Self::recursive_parent_remove(parent.clone(), granchild.clone());
        }
    }

    fn transitive_reduce_iter(curr_node: Rc<Link<T>>) {
        for child in curr_node.0.borrow().children.iter() {
            for granchild in child.0.borrow().children.iter() {
                Self::recursive_parent_remove(curr_node.clone(), granchild.clone());
            }
        }
    }

    // TODO: see if we can reduce the amount of rc cloning happening
    pub fn transitive_reduce(&self) {
        for root in self.roots.iter() {
            Self::transitive_reduce_iter(root.clone())
        }
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
