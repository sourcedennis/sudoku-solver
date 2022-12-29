use super::*;

/// cells: (0,0), (1,0), (2,0)
fn row_frag0( ) -> RowFragment {
  RowFragment { y: 0, frag_x: 0 }
}

/// cells: (3,4), (4,4), (5,4)
fn row_frag13( ) -> RowFragment {
  RowFragment { y: 4, frag_x: 1 }
}

/// cells: (0,0), (0,1), (0,2)
fn col_frag0( ) -> ColFragment {
  ColFragment { x: 0, frag_y: 0 }
}

/// cells: (4,3), (4,4), (4,5)
fn col_frag13( ) -> ColFragment {
  ColFragment { x: 4, frag_y: 1 }
}

#[test]
fn test_row_frag0_contained( ) {
  let frag = row_frag0( ); // cells: (0,0), (1,0), (2,0)
  let mut it_contained = frag.cells( );
  assert_eq!( it_contained.next( ), Some( 0 * 9 + 0 ) );
  assert_eq!( it_contained.next( ), Some( 0 * 9 + 1 ) );
  assert_eq!( it_contained.next( ), Some( 0 * 9 + 2 ) );
  assert_eq!( it_contained.next( ), None );
}

#[test]
fn test_row_frag0_row( ) {
  let frag = row_frag0( ); // cells: (0,0), (1,0), (2,0)
  let mut it_ext_row = frag.ext_adjacent_cells( );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 3 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 5 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 6 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 7 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 8 ) );
  assert_eq!( it_ext_row.next( ), None );
}

#[test]
fn test_row_frag0_block( ) {
  let frag = row_frag0( ); // cells: (0,0), (1,0), (2,0)
  let mut it_ext_block = frag.ext_block_cells( );
  assert_eq!( it_ext_block.next( ), Some( 1 * 9 + 0 ) );
  assert_eq!( it_ext_block.next( ), Some( 1 * 9 + 1 ) );
  assert_eq!( it_ext_block.next( ), Some( 1 * 9 + 2 ) );
  assert_eq!( it_ext_block.next( ), Some( 2 * 9 + 0 ) );
  assert_eq!( it_ext_block.next( ), Some( 2 * 9 + 1 ) );
  assert_eq!( it_ext_block.next( ), Some( 2 * 9 + 2 ) );
}

#[test]
fn test_row_frag13_contained( ) {
  let frag = row_frag13( ); // cells: (3,4), (4,4), (5,4)
  let mut it_contained = frag.cells( );
  assert_eq!( it_contained.next( ), Some( 4 * 9 + 3 ) );
  assert_eq!( it_contained.next( ), Some( 4 * 9 + 4 ) );
  assert_eq!( it_contained.next( ), Some( 4 * 9 + 5 ) );
  assert_eq!( it_contained.next( ), None );
}

#[test]
fn test_row_frag13_row( ) {
  let frag = row_frag13( ); // cells: (3,4), (4,4), (5,4)
  let mut it_ext_row = frag.ext_adjacent_cells( );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 6 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 7 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 8 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 1 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 2 ) );
  assert_eq!( it_ext_row.next( ), None );
}

#[test]
fn test_row_frag13_block( ) {
  let frag = row_frag13( ); // cells: (3,4), (4,4), (5,4)
  let mut it_ext_block = frag.ext_block_cells( );
  assert_eq!( it_ext_block.next( ), Some( 5 * 9 + 3 ) );
  assert_eq!( it_ext_block.next( ), Some( 5 * 9 + 4 ) );
  assert_eq!( it_ext_block.next( ), Some( 5 * 9 + 5 ) );
  assert_eq!( it_ext_block.next( ), Some( 3 * 9 + 3 ) );
  assert_eq!( it_ext_block.next( ), Some( 3 * 9 + 4 ) );
  assert_eq!( it_ext_block.next( ), Some( 3 * 9 + 5 ) );
}

#[test]
fn test_col_frag0_contained( ) {
  let frag = col_frag0( ); // cells: (0,0), (0,1), (0,2)
  let mut it_contained = frag.cells( );
  assert_eq!( it_contained.next( ), Some( 0 * 9 + 0 ) );
  assert_eq!( it_contained.next( ), Some( 1 * 9 + 0 ) );
  assert_eq!( it_contained.next( ), Some( 2 * 9 + 0 ) );
  assert_eq!( it_contained.next( ), None );
}

#[test]
fn test_col_frag0_col( ) {
  let frag = col_frag0( ); // cells: (0,0), (0,1), (0,2)
  let mut it_ext_row = frag.ext_adjacent_cells( );
  assert_eq!( it_ext_row.next( ), Some( 3 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 4 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 5 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 6 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 7 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), Some( 8 * 9 + 0 ) );
  assert_eq!( it_ext_row.next( ), None );
}

#[test]
fn test_col_frag0_block( ) {
  let frag = col_frag0( ); // cells: (0,0), (0,1), (0,2)
  let mut it_ext_block = frag.ext_block_cells( );
  assert_eq!( it_ext_block.next( ), Some( 0 * 9 + 1 ) );
  assert_eq!( it_ext_block.next( ), Some( 1 * 9 + 1 ) );
  assert_eq!( it_ext_block.next( ), Some( 2 * 9 + 1 ) );
  assert_eq!( it_ext_block.next( ), Some( 0 * 9 + 2 ) );
  assert_eq!( it_ext_block.next( ), Some( 1 * 9 + 2 ) );
  assert_eq!( it_ext_block.next( ), Some( 2 * 9 + 2 ) );
}

#[test]
fn test_col_frag13_contained( ) {
  let frag = col_frag13( ); // cells: (4,3), (4,4), (4,5)
  let mut it_contained = frag.cells( );
  assert_eq!( it_contained.next( ), Some( 3 * 9 + 4 ) );
  assert_eq!( it_contained.next( ), Some( 4 * 9 + 4 ) );
  assert_eq!( it_contained.next( ), Some( 5 * 9 + 4 ) );
  assert_eq!( it_contained.next( ), None );
}

#[test]
fn test_col_frag13_col( ) {
  let frag = col_frag13( ); // cells: (4,3), (4,4), (4,5)
  let mut it_ext_row = frag.ext_adjacent_cells( );
  assert_eq!( it_ext_row.next( ), Some( 6 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 7 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 8 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 0 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 1 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), Some( 2 * 9 + 4 ) );
  assert_eq!( it_ext_row.next( ), None );
}

#[test]
fn test_col_frag13_block( ) {
  let frag = col_frag13( ); // cells: (4,3), (4,4), (4,5)
  let mut it_ext_block = frag.ext_block_cells( );
  assert_eq!( it_ext_block.next( ), Some( 3 * 9 + 5 ) );
  assert_eq!( it_ext_block.next( ), Some( 4 * 9 + 5 ) );
  assert_eq!( it_ext_block.next( ), Some( 5 * 9 + 5 ) );
  assert_eq!( it_ext_block.next( ), Some( 3 * 9 + 3 ) );
  assert_eq!( it_ext_block.next( ), Some( 4 * 9 + 3 ) );
  assert_eq!( it_ext_block.next( ), Some( 5 * 9 + 3 ) );
}
