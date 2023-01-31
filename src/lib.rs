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

    fn count_edges_iter(curr_node: Rc<Link<T>>, nodes_visited: &mut BTreeSet<Rc<Link<T>>>) -> u32 {
        let mut edge_count = 0;

        for child in curr_node.0.borrow().children.iter() {
            edge_count += 1;

            if !nodes_visited.contains(child) {
                edge_count += Self::count_edges_iter(child.clone(), nodes_visited);
            }
        }

        nodes_visited.insert(curr_node);

        edge_count
    }

    pub fn count_edges(&self) -> u32 {
        let mut edge_count = 0;
        let mut nodes_visited: BTreeSet<Rc<Link<T>>> = BTreeSet::new();

        for root in self.roots.iter() {
            edge_count += Self::count_edges_iter(root.clone(), &mut nodes_visited);
        }

        edge_count
    }

    fn recursive_parent_remove(parent: Rc<Link<T>>, child: Rc<Link<T>>) {
        Self::remove_edge(parent.clone(), child.clone());
        for granchild in child.0.borrow().children.iter() {
            Self::recursive_parent_remove(parent.clone(), granchild.clone());
        }
    }

    fn transitive_reduce_iter(curr_node: Rc<Link<T>>) {
        let children = curr_node.0.borrow().children.clone(); // FIXME: would be nice to not have to clone here

        for child in children.iter() {
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
    use super::*;

    #[test]
    fn test_transitive_reduce() {
        let mut dag: Dag<u32> = Dag::new();

        let node1 = dag.add_node(1);
        let node2 = dag.add_node(2);
        let node3 = dag.add_node(3);
        let node4 = dag.add_node(4);
        let node5 = dag.add_node(5);

        dag.add_edge(node1.clone(), node2.clone());
        dag.add_edge(node1.clone(), node3.clone());
        dag.add_edge(node1.clone(), node4.clone());
        dag.add_edge(node1.clone(), node5.clone());

        dag.add_edge(node2.clone(), node4.clone());
        dag.add_edge(node3.clone(), node4.clone());
        dag.add_edge(node3.clone(), node5.clone());
        dag.add_edge(node4.clone(), node5.clone());

        assert_eq!(dag.count_edges(), 8);

        dag.transitive_reduce();

        assert_eq!(dag.count_edges(), 5);
    }
}
