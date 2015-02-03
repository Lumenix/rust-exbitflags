#![feature(hash)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate exbitflags;

#[test]
fn test_iota() {
	// empty test
    assert_eq!( 0, macro_seqc!() );

    // expr
    assert_eq!( 1, macro_seqc!( 9 ) );
    assert_eq!( 3, macro_seqc!( 9, 9, 9 ) );

    // ident - matched by expr
    assert_eq!( 1, macro_seqc!( Nine ) );
    assert_eq!( 3, macro_seqc!( Nine, Nine, Nine ) );

    // ty - matched by expr
    assert_eq!( 1, macro_seqc!( Option::<int> ) );
    assert_eq!( 3, macro_seqc!( Option::<int>, Option::<int>, Option::<int> ) );

    // block - matched by expr
    assert_eq!( 1, macro_seqc!( { 9; 9 } ) );
    assert_eq!( 3, macro_seqc!( { 9; 9 }, { 9; 9 }, { 9; 9 } ) );

    // pat
    assert_eq!( 1, macro_seqc!( foo @ _ ) );
    assert_eq!( 3, macro_seqc!( foo @ _,foo @ _,foo @ _ ) );
}

ebf!( Op1: u32 { ADD = 1, DELETE = 2, MODIFY = 4 } );
ebf!( Flag: u32 { FLAG_1, FLAG_2, FLAG_3 } );

#[test]
fn test_ebf() {
	assert_eq!( ADD.bits,		0x1 );
	assert_eq!( DELETE.bits,	0x2 );
	assert_eq!( MODIFY.bits,	0x4 );

	assert_eq!( FLAG_1.bits,	0x1 );
	assert_eq!( FLAG_2.bits,	0x2 );
	assert_eq!( FLAG_3.bits,	0x4 );
}