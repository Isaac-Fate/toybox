#[derive(Debug)]
pub struct LinkedList<T> {
    pub value: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

pub struct LinkedListNode<T> {
    pub value: T,
    pub next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::<i32>::new();

        println!("list: {:?}", list);
    }
}
