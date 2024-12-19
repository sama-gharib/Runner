
pub trait Pair {
	type First;
	type Second;

	fn as_pair(&self) -> (Self::First, Self::Second);
}
