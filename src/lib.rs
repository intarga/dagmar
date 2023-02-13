use std::collections::BTreeSet;

pub struct Node<T> {
    pub elem: T,
    pub children: BTreeSet<NodeId>,
}

type NodeId = usize;

pub struct Dag<T: Ord> {
    pub roots: BTreeSet<NodeId>,
    pub nodes: Vec<Node<T>>,
}

impl<T: Ord> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            children: BTreeSet::new(),
        }
    }
}

impl<T: Ord> Dag<T> {
    pub fn new() -> Self {
        Dag {
            roots: BTreeSet::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, elem: T) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node::new(elem));

        self.roots.insert(index);

        index
    }

    pub fn add_edge(&mut self, parent: NodeId, child: NodeId) {
        // TODO: we can do better than unwrapping here
        self.nodes.get_mut(parent).unwrap().children.insert(child);

        self.roots.remove(&child);
    }

    pub fn add_node_with_children(&mut self, elem: T, children: Vec<NodeId>) {
        let new_node = self.add_node(elem);

        for child in children.into_iter() {
            self.add_edge(new_node, child)
        }
    }

    // NOTE: this doesn't add to roots when a node no longer has parents,
    // only for use in transitive reduce.
    fn remove_edge(&mut self, parent: NodeId, child: NodeId) {
        // TODO: we can do better than unwrapping here
        self.nodes.get_mut(parent).unwrap().children.remove(&child);
    }

    fn count_edges_iter(&self, curr_node: NodeId, nodes_visited: &mut BTreeSet<NodeId>) -> u32 {
        let mut edge_count = 0;

        for child in self.nodes.get(curr_node).unwrap().children.iter() {
            edge_count += 1;

            if !nodes_visited.contains(child) {
                edge_count += self.count_edges_iter(*child, nodes_visited);
            }
        }

        nodes_visited.insert(curr_node);

        edge_count
    }

    pub fn count_edges(&self) -> u32 {
        let mut edge_count = 0;
        let mut nodes_visited: BTreeSet<NodeId> = BTreeSet::new();

        for root in self.roots.iter() {
            edge_count += self.count_edges_iter(root.clone(), &mut nodes_visited);
        }

        edge_count
    }

    fn recursive_parent_remove(&mut self, parent: NodeId, child: NodeId) {
        self.remove_edge(parent, child);
        for granchild in self.nodes.get(child).unwrap().children.clone().iter() {
            self.recursive_parent_remove(parent, *granchild);
        }
    }

    fn transitive_reduce_iter(&mut self, curr_node: NodeId) {
        let children = self.nodes.get(curr_node).unwrap().children.clone(); // FIXME: would be nice to not have to clone here

        for child in children.iter() {
            for granchild in self.nodes.get(*child).unwrap().children.clone().iter() {
                self.recursive_parent_remove(curr_node, *granchild);
            }
        }

        for child in children.iter() {
            self.transitive_reduce_iter(*child);
        }
    }

    // TODO: see if we can reduce the amount of rc cloning happening
    pub fn transitive_reduce(&mut self) {
        for root in self.roots.clone().iter() {
            self.transitive_reduce_iter(root.clone())
        }
    }

    fn cycle_check_iter(&self, curr_node: NodeId, ancestors: &mut Vec<NodeId>) -> bool {
        if ancestors.contains(&curr_node) {
            return true;
        }

        ancestors.push(curr_node.clone());

        for child in self.nodes.get(curr_node).unwrap().children.iter() {
            if self.cycle_check_iter(*child, ancestors) {
                return true;
            }
        }

        ancestors.pop();

        false
    }

    pub fn cycle_check(&self) -> bool {
        let mut ancestors: Vec<NodeId> = Vec::new();

        for root in self.roots.iter() {
            if self.cycle_check_iter(*root, &mut ancestors) {
                return true;
            }
        }

        false
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

        dag.add_edge(node1, node2);
        dag.add_edge(node1, node3);
        dag.add_edge(node1, node4);
        dag.add_edge(node1, node5);

        dag.add_edge(node2, node4);
        dag.add_edge(node3, node4);
        dag.add_edge(node3, node5);
        dag.add_edge(node4, node5);

        assert_eq!(dag.count_edges(), 8);
        assert!(dag.nodes.get(node1).unwrap().children.contains(&node4));
        assert!(dag.nodes.get(node1).unwrap().children.contains(&node5));
        assert!(dag.nodes.get(node3).unwrap().children.contains(&node5));

        dag.transitive_reduce();

        assert_eq!(dag.count_edges(), 5);
        assert!(!dag.nodes.get(node1).unwrap().children.contains(&node4));
        assert!(!dag.nodes.get(node1).unwrap().children.contains(&node5));
        assert!(!dag.nodes.get(node3).unwrap().children.contains(&node5));
    }

    #[test]
    fn test_cycle_check() {
        let mut good_dag: Dag<u32> = Dag::new();

        let node1 = good_dag.add_node(1);
        let node2 = good_dag.add_node(2);
        let node3 = good_dag.add_node(3);
        let node4 = good_dag.add_node(4);

        good_dag.add_edge(node1, node2);
        good_dag.add_edge(node1, node3);
        good_dag.add_edge(node2, node4);
        good_dag.add_edge(node3, node4);

        assert!(!good_dag.cycle_check());

        let mut bad_dag: Dag<u32> = Dag::new();

        let node1 = bad_dag.add_node(1);
        let node2 = bad_dag.add_node(2);
        let node3 = bad_dag.add_node(3);
        let node4 = bad_dag.add_node(4);

        bad_dag.add_edge(node1, node2);
        bad_dag.add_edge(node1, node3);
        bad_dag.add_edge(node2, node4);
        bad_dag.add_edge(node4, node3);
        bad_dag.add_edge(node3, node2);

        assert!(bad_dag.cycle_check());
    }
}
