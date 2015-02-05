/**
 * Helper! Given a list of identifiers,
 * it creates flags for each one according to the series defined by: 2^n.
 * This causes all flags to be exclusive.
 */
#[macro_export]
macro_rules! ebf_flags {
	( $val:expr, $flag:ident ) => {
		const $flag:u32 = 1 << ($val);
	};

	( $val:expr, $flag:ident, $($tail:ident), + ) => {
		ebf_flags! { $val, $flag  }
		ebf_flags! { 1 + $val, $($tail), + }
	};

	( $val:expr, $( $flags:ident ), + ) => {
		ebf_flags! { $val, $( $flags ), + }
	};
}

macro_rules! ebf_flags2 {
	( $val:expr, $flag:ident ) => {
		const $flag:u32 = 1 << ($val);
	};

	( $val:expr, $flag:ident, $($tail:ident), + ) => {
		ebf_flags2! { $val, $flag }
		ebf_flags2! { $val + 1, $($tail), + }
	};
}

ebf_flags! { 0, A, B, C }
ebf_flags2! { 0, D, E, F }