use std::collections::BTreeSet;
use std::rc::Rc;

pub struct Dag<T> {
    roots: BTreeSet<Rc<Node<T>>>,
}

struct Node<T> {
    elem: T,
    children: BTreeSet<Rc<Node<T>>>,
}

impl<T> Dag<T> {
    pub fn new() -> Self {
        Dag {
            roots: BTreeSet::new(),
        }
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
