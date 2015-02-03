#![crate_type = "lib"]
#![crate_name = "exbitflags"]

#![feature(macro_rules)]
#[doc(hidden)]

/// Helper! Expands to:
/// ```
/// static $flag = $v
/// ```
#[macro_export]
macro_rules! ebf_flag(
	($flag:ident, $v: expr) => (
		static $flag = $v,
	);
)

/// Helper! Given a list of identifiers,
/// it creates flags for each one according to the series defined by: 2^n.
/// This causes all flags to be exclusive.
#[macro_export]
macro_rules! ebf_flags {
	($flag:ident) => (
		ebf_flag!( $flag, 0x01 );
	);
	($flag:ident, $($tail:expr), +) => (
		ebf_flags!( 0x01, $flag, $($tail), + );
    );
	($val:expr, $flag:ident, $($tail:expr), + ) => (
		ebf_flag!( $flag, $val );
		ebf_flags!( $val << 1, $($tail), + );
	);
}

/// Creates bitfields with $name, with the type $t with a list of flags.
/// Example:
/// ```
/// ebf!( Operations, u32, ADD, DELETE, MODIFY );
#[macro_export]
macro_rules! ebf {
	($name:ident, $t:ty, $flag:ident, $($tail:expr), +) => (
		bitflags!( flags $name: $t ) {
			ebf_flags!( $flag, $($tail:expr), + );
		}
	);
}