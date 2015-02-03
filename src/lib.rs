#![crate_type = "dylib"]
#![crate_name = "exbitflags"]

#[macro_use]
extern crate bitflags;

//================================================================================
// macro_seqc!
//================================================================================

/**
 * General helper for macros, "returns" the number of arguments passed.
 *
 * Example:
 * ````rust
 * assert_eq!( 3, macro_seqc!( 9, 9, 9 ) );
 * ```
 */
#[macro_export]
macro_rules! macro_seqc {
    ( $( $thing:expr ),+ ) => { macro_seqc_expr!(1, $($thing),*) };
    ( $( $thing:pat ),+ ) => { macro_seqc_pat!(1, $($thing),*) };
    () => { 0 };
}

/**
 * Helper for macro_seqc!
 */
#[macro_export]
macro_rules! macro_seqc_expr {
    ( $id:expr, $head:expr, $( $tail:expr ),* ) => ({
    	macro_seqc_expr!( $id + 1, $( $tail ),* )
    });
    ( $id:expr, $last:expr ) => ( $id );
}

/**
 * Helper for macro_seqc!
 */
#[macro_export]
macro_rules! macro_seqc_pat {
    ( $id:expr, $head:pat, $( $tail:pat ),* ) => ({
    	macro_seqc_pat!( $id + 1, $( $tail ),* )
    });
    ( $id:expr, $last:pat ) => ( $id );
}

//================================================================================
// ebf!
//================================================================================

macro_rules! ebf_excl {
	( $( $thing:expr ),+ ) => { 1 << (macro_seqc!( $thing )) };
}

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
		ebf_flags!( $val, $flag );
		ebf_flags!( 1 + $val, $($tail), + );
	};

	( $val:expr, $( $flags:ident ), + ) => {
		ebf_flags!( $val, $( $flags ), + );
	};
}

/**
 * Creates bitfields with $name, with the type $t with a list of flags.
 *
 * Example:
 * ````rust
 * ebf!( Operations, u32, ADD, DELETE, MODIFY );
 * ```
 */
#[macro_export]
macro_rules! ebf {
	( $( #[$attr:meta] )* $BitFlags:ident: $T:ty {
		$( $( #[$Flag_attr:meta] )* $Flag:ident = $value:expr ),+
	} ) => {
		bitflags! {
			$( #[$attr] )*
			flags $BitFlags: $T {
				$( $( #[$Flag_attr] )* const $Flag = $value ),+
			}
		}
	};
	( $( #[$attr:meta] )* $BitFlags:ident: $T:ty {
		$( $( #[$Flag_attr:meta] )* $Flag:ident = $value:expr ),+,
	} ) => {
		bitflags! {
			$( #[$attr] )*
			flags $BitFlags: $T {
				$( $( #[$Flag_attr] )* const $Flag = $value ),+
			}
		}
	};

	( $( #[$attr:meta] )* $BitFlags:ident: $T:ty {
		$( $( #[$Flag_attr:meta] )* $Flag:ident ),+
	} ) => {
		bitflags! {
			$( #[$attr] )*
			flags $BitFlags: $T {
				$( $( #[$Flag_attr] )* $Flag = ebf_excl!( $( $( #[$Flag_attr] )* $Flag ),+ ) ),+
			}
		}
	};
	( $( #[$attr:meta] )* $BitFlags:ident: $T:ty {
		$( $( #[$Flag_attr:meta] )* $Flag:ident ),+,
	} ) => {
		bitflags! {
			$( #[$attr] )*
			flags $BitFlags: $T {
				$( $( #[$Flag_attr] )* const $Flag = $value ),+
			}
		}
	};
}