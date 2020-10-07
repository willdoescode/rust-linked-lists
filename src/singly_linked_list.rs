use std::ops::Deref;

pub struct Node<T> {
	pub elem: T,
	pub next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
	pub fn new(elem: T) -> Self {
		Node {
			elem,
			next: None
		}
	}
	pub fn push_front(&mut self, elem: T) {
		self.next = Some(Box::new(Node::new(elem)))
	}
	pub fn push_back(self, elem: T) -> Node<T> {
		Node {
			elem,
			next: Some(Box::new(self))
		}
	}
}