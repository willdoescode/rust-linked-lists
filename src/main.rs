use std::ops::Deref;
use crate::singly_linked_list::Node;

mod singly_linked_list;

fn main() {
	let mut i = singly_linked_list::Node::new(1);
	i = i.push_back(5);
	i.push_front(8);
	println!("{}", i.elem);
	println!("{}", i.next.unwrap().deref().elem);
}
