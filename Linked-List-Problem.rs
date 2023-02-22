use std::iter::{self, FromIterator};

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head : None}
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;
        while let Some(x) = current_node {
            size += 1;
            current_node = &x.next;
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            elem: _element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node: Box<Node<T>>|{
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node: &Box<Node<T>>|{
            &node.elem
        })
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            ret.push(x);
        }
        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter.into_iter().fold(SimpleLinkedList::new(), |mut l, e| {
            l.push(e);
            l
        })
    }
}
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = iter::from_fn(|| _linked_list.pop()).collect::<Vec<_>>();
        vec.reverse();
        vec
    }
}
