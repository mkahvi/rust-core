/// For things that can be empty.
pub trait Empty {
	fn is_empty(&self) -> bool;
	fn empty(&mut self);
}
