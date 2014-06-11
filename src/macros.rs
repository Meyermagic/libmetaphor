

macro_rules! alternative(
	($in_expr:expr, $pat:pat, $out_true_expr:expr, $out_false_expr:expr) => (
		match $in_expr {
			$pat => $out_true_expr,
			_ => $out_false_expr,
		}
	);
)

macro_rules! is_match(
	($expr:expr, $pat:pat) => (
		alternative!($expr, $pat, true, false)
	);
)

macro_rules! extract(
	($in_expr:expr, $pat:pat, $out_expr:expr) => (
		alternative!($in_expr, $pat, Some($out_expr), None)
	);
)

