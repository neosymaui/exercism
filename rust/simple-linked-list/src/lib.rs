pub struct SimpleLinkedList<T: Copy + Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy + Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut option = &self.head;

        while option.is_some() {
            length += 1;
            option = &option.as_ref().unwrap().next;
        }
        length
    }

    pub fn push(&mut self, _element: T) {
        let n = Node::new(_element);
        let mut last = &mut self.head;

        while last.is_some() {
            last = &mut last.as_mut().unwrap().next;
        }
        *last = Some(Box::new(n));
    }

    pub fn pop(&mut self) -> Option<T> {
        let length = self.len();
        if length == 0 {
            None
        } else if length == 1 {
            let n: T = self.head.as_mut().unwrap().data.clone();
            self.head = None;
            Some(n)
        } else {
            let mut cur = &mut self.head;

            for _l in 1..length - 1 {
                cur = &mut cur.as_mut().unwrap().next;
            }

            let n: T = cur.as_mut().unwrap().next.as_mut().unwrap().data.clone();
            cur.as_mut().unwrap().next = None;
            Some(n)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let length = self.len();
        if length == 0 {
            None
        } else {
            let mut last = &self.head;

            for _l in 0..length - 1 {
                last = &last.as_ref().unwrap().next;
            }

            Some(&last.as_ref().unwrap().data)
        }
    }
}

impl<T: Copy + Clone> SimpleLinkedList<T> {
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut rev = SimpleLinkedList::new();

        while self.len() > 0 {
            match &self.pop() {
                Some(x) => rev.push(x.clone()),
                None => (),
            }
        }
        rev
    }
}

impl<'a, T: Copy + Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut l = SimpleLinkedList::new();
        for i in _item.iter() {
            l.push(i.clone());
        }
        l
    }
}

impl<T: Copy + Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut cur = &self.head;
        let mut res = vec![];
        while cur.is_some() {
            res.push(cur.as_ref().unwrap().data);
            cur = &cur.as_ref().unwrap().next;
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
        }
    }
}
