use std::rc::Rc;

pub struct Dag<T> {
    roots: Vec<Link<T>>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    children: Vec<Link<T>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
