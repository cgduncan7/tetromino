use std::{cell::RefCell, cmp::max, fmt::Debug, rc::Rc};

pub type OptionalRef<I> = Option<Rc<RefCell<I>>>;

#[derive(Clone, Debug, Eq)]
pub struct DoublyLinkedListNode<T: Debug> {
    pub prev: OptionalRef<DoublyLinkedListNode<T>>,
    pub next: OptionalRef<DoublyLinkedListNode<T>>,
    pub id: i32,
    pub contents: Box<T>,
}

impl<T: Debug> PartialEq for DoublyLinkedListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Debug> DoublyLinkedListNode<T> {
    pub fn new(contents: T) -> Self {
        Self {
            prev: None,
            next: None,
            id: rand::random::<i32>(),
            contents: Box::new(contents),
        }
    }

    pub fn set_prev(&mut self, prev: OptionalRef<DoublyLinkedListNode<T>>) {
        self.prev = prev;
    }

    pub fn set_next(&mut self, next: OptionalRef<DoublyLinkedListNode<T>>) {
        self.next = next;
    }
}

#[derive(Clone, Debug, Eq)]
pub struct CircularDoublyLinkedList<T: Debug + Eq + PartialEq> {
    pub size: usize,
    pub head: OptionalRef<DoublyLinkedListNode<T>>,
    pub tail: OptionalRef<DoublyLinkedListNode<T>>,

    // iterator values
    iterator_index: usize,
    iterator_current: OptionalRef<DoublyLinkedListNode<T>>,
}

impl<T: Debug + Eq + PartialEq> CircularDoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
            iterator_index: 0,
            iterator_current: None,
        }
    }

    pub fn find(&self, contents_to_find: T) -> OptionalRef<DoublyLinkedListNode<T>> {
        if self.head.is_none() {
            return None;
        }

        let mut current_node_ref = self.head.clone().unwrap();

        for _ in 0..self.size {
            if current_node_ref.borrow().contents.as_ref() == &contents_to_find {
                return Some(current_node_ref);
            }
            let next_current_node = current_node_ref.borrow().next.as_ref().unwrap().clone();
            current_node_ref = next_current_node;
        }

        None
    }

    pub fn insert_head(&mut self, new_head: T) {
        self.insert_at(new_head, 0);
    }

    pub fn insert_tail(&mut self, new_tail: T) {
        self.insert_at(new_tail, self.size);
    }

    pub fn insert_at(&mut self, new_item: T, idx: usize) {
        let new_node = Rc::new(RefCell::new(DoublyLinkedListNode::new(new_item)));
        let mut current_idx: usize = 0;
        if self.head.is_none() {
            new_node.borrow_mut().set_prev(Some(new_node.clone()));
            new_node.borrow_mut().set_next(Some(new_node.clone()));
        } else {
            let idx = max(idx, self.size);
            let mut current_node_ref = self.head.clone().unwrap();
            // 0, 1, 2, ..., n
            // a  b  c  ...  N
            while current_idx + 1 < idx {
                let next_current_node = current_node_ref.borrow().next.as_ref().unwrap().clone();
                current_node_ref = next_current_node;
                current_idx += 1;
            }

            let curr_prev = current_node_ref.borrow().prev.as_ref().unwrap().clone();

            new_node.borrow_mut().set_prev(Some(curr_prev.clone()));
            new_node
                .borrow_mut()
                .set_next(Some(current_node_ref.clone()));

            curr_prev.borrow_mut().set_next(Some(new_node.clone()));
            current_node_ref
                .borrow_mut()
                .set_prev(Some(new_node.clone()));
        }

        self.size += 1;

        if idx == 0 {
            self.head = Some(new_node.clone());
        }

        if idx == self.size {
            self.tail = Some(new_node.clone());
        }
    }
}

impl<T: Debug + Eq + PartialEq> PartialEq for CircularDoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.head == other.head && self.tail == other.tail
    }
}

impl<T: Debug + Eq + PartialEq> Iterator for CircularDoublyLinkedList<T> {
    type Item = Rc<RefCell<DoublyLinkedListNode<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_index >= self.size {
            return None;
        }

        let to_return = match &self.iterator_current {
            None => self.head.clone(),
            Some(ic) => ic.borrow().next.clone(),
        };

        self.iterator_index += 1;
        self.iterator_current = to_return.clone();

        to_return
    }
}
#[cfg(test)]
mod test {
    use super::CircularDoublyLinkedList;

    #[test]
    fn test_linked_list() {
        let mut dll = CircularDoublyLinkedList::new();
        dll.insert_head('a');
        dll.insert_tail('b');
        dll.insert_at('c', 1);
        dll.insert_at('d', 2);

        let mut current_node = dll.head.clone();

        let mut results: Vec<char> = Vec::new();
        for _ in 0..dll.size + 1 {
            let c = current_node.unwrap();
            let borrowed_c = c.borrow();
            results.push(*borrowed_c.contents);
            current_node = borrowed_c.next.clone();
        }

        assert_eq!(vec!['a', 'c', 'd', 'b', 'a'], results);

        let mut results: Vec<char> = Vec::new();
        for n in dll {
            results.push(*n.borrow().contents);
        }
        assert_eq!(vec!['a', 'c', 'd', 'b'], results);
    }
}
