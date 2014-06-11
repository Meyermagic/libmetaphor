use std::cmp::min;

#[inline]
fn xyw(x: uint, y: uint, w: uint) -> uint { x + y * w }

//O(a*b) space LCS
pub fn lcs<E: Eq>(a: &[E], b: &[E]) -> Vec<(uint, uint)> {
	let width = a.len() + 1;
	let height = b.len() + 1;

	let xy = |x: uint, y: uint| -> uint { xyw(x, y, width) };

	let mut lengths: Vec<uint> = Vec::from_elem(width * height, 0u);

	for x in range(0, a.len()) {
		for y in range(0, b.len()) {
			if a.get(x) == b.get(y) {
				*lengths.get_mut(xy(x+1, y+1)) = *lengths.get(xy(x, y)) + 1;
			} else if lengths.get(xy(x+1, y)) > lengths.get(xy(x, y+1)) {
				*lengths.get_mut(xy(x+1, y+1)) = *lengths.get(xy(x+1, y));
			} else {
				*lengths.get_mut(xy(x+1, y+1)) = *lengths.get(xy(x, y+1));
			}
		}
	}

	let mut seq = Vec::with_capacity(*lengths.get(a.len() + b.len() * width));
	let mut x = a.len();
	let mut y = b.len();

	while x != 0 && y != 0 {
		if lengths.get(xy(x, y)) == lengths.get(xy(x-1, y)) {
			x -= 1;
		} else if lengths.get(xy(x, y)) == lengths.get(xy(x, y-1)) {
			y -= 1;
		} else {
			seq.push((x-1, y-1));
			x -= 1;
			y -= 1;
		}
	}

	seq.reverse();

	return seq;
}

//Counts the number of (non-overlapping) elements matching at the start and end of each slice
pub fn common_affix<E: Eq>(old: &[E], new: &[E]) -> (uint, uint) {
	let shorter = min(old.len(), new.len());

	let mut common_prefix = 0;
	for i in range(0, shorter) {
		if old.get(i) != new.get(i) {
			break;
		}
		common_prefix += 1;
	}

	let mut common_suffix = 0;
	for i in range(0, shorter - common_prefix) {
		if old.get((old.len() - 1) - i) != new.get((new.len() - 1) - i) {
			break;
		}
		common_suffix += 1;
	}

	return (common_prefix, common_suffix);
}