#![feature(return_position_impl_trait_in_trait)]

use std::fmt::Display;


fn main() {
  let mut sudoku =
    Sudoku::from(
      [0,3,4, 1,0,0, 0,0,0
      ,0,0,2, 0,0,8, 0,0,0
      ,0,7,0, 0,0,0, 0,0,4

      ,0,0,0, 0,2,0, 3,0,7
      ,0,6,0, 0,8,4, 0,0,5
      ,0,0,0, 0,9,0, 0,2,0

      ,9,0,0, 0,0,0, 0,4,0
      ,6,2,5, 0,0,0, 0,8,9
      ,0,0,0, 0,6,9, 0,1,0]
    );

  println!( "== INITIAL" );
  println!( "{}", sudoku );
  println!( );

  if let Ok( () ) = solve( &mut sudoku ) {
    if sudoku.is_solved( ) {
      println!( "== SOLVED" );
    } else {
      println!( "== SOLVING FAILED" );
    }
  } else {
    // this should only happen for incorrect sudokus, or sudokus with multiple
    // solutions, which we don't support.
    println!( "== UNSOLVABLE" );
  }
}


/// 9 bit flags, each representing whether a value is possible (1) or not (0)
#[derive(Copy,Clone)]
struct SudokuCell( usize );

impl SudokuCell {
  /// Constructs a new cell where all values (1-9) are possible
  pub fn new_all( ) -> SudokuCell {
    SudokuCell( 0b111_111_111 )
  }

  /// Constructs a cell where only a single constant value is possible
  pub fn new_const( v: usize  ) -> SudokuCell {
    debug_assert!( 1 <= v && v <= 9 );
    SudokuCell( 1 << ( v - 1 ) )
  }

  /// Returns the number of possible values
  pub fn num_possibilities( &self ) -> usize {
    self.0.count_ones( ) as usize
  }

  /// Returns `true` iff the cell is solved. i.e., it has a *single* possible
  /// value.
  pub fn is_solved( &self ) -> bool {
    self.num_possibilities( ) == 1
  }

  pub fn solution( &self ) -> Option< usize > {
    if self.is_solved( ) {
      Some( self.0.trailing_zeros( ) as usize + 1 )
    } else {
      None
    }
  }
}

#[derive(Clone)]
struct Sudoku( [SudokuCell; 9*9] );

enum Error {
  Unsolvable
}

impl Sudoku {
  pub fn is_solved( &self ) -> bool {
    self.0.iter( ).all( |x| x.is_solved( ) )
  }

  /// Returns that bitmask of values that necessarily exist in the 3-cell
  /// fragment.
  pub fn frag_necessary< F: ThreeCellFragment >( &self, frag: &F ) -> usize {
    // values that are possible outside our fragment, in the row
    let row_ext_possible: usize =
      frag.ext_adjacent_cells( )
        .map( |idx| self.0[ idx ].0 )
        .fold( 0b000_000_000,  |acc, opts| ( acc | opts ) );

    // values that are possible outside our fragment, in the block
    let block_ext_possible =
      frag.ext_block_cells( )
        .map( |idx| self.0[ idx ].0 )
        .fold( 0b000_000_000,  |acc, opts| ( acc | opts ) );

    // things that are *impossible outside our fragment*, surely have to be
    // inside our fragment.
    ( !row_ext_possible | !block_ext_possible | self.frag_solutions( frag ) ) & 0b111_111_111
  }

  /// Returns a bitmask with bits set for solved cells within the fragment.
  pub fn frag_solutions< F: ThreeCellFragment >( &self, frag: &F ) -> usize {
    frag.cells( )
        .map( |cell_idx| self.0[ cell_idx ] )
        .filter( |cell| cell.is_solved( ) )
        .fold( 0b000_000_000,  |acc, opts| ( acc | opts.0 ) )
  }
}

impl From< [u8; 9*9] > for Sudoku {
  fn from( cells: [u8; 9*9] ) -> Sudoku {
    // We construct a sudoku from the cells. 0 represents that everything is
    // possible. Other values are constants.
    let mut out_cells = [SudokuCell::new_all(); 9*9];
    for i in 0..9*9 {
      if cells[ i ] != 0 {
        out_cells[ i ] = SudokuCell::new_const( cells[ i ] as usize );
      }
    }
    Sudoku( out_cells )
  }
}

impl Display for Sudoku {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..9 {
      for x in 0..9 {
        if let Some( v ) = self.0[ y * 9 + x ].solution( ) {
          write!( f, "{}", v )?;
        } else {
          write!( f, "_" )?;
        }
        if x == 2 || x == 5 { // diagonal separator (after 3 columns)
          write!( f, " " )?;
        }
      }
      writeln!( f )?;
      if y == 2 || y == 5 { // horizontal separator (after 3 rows)
        writeln!( f )?;
      }
    }
    Ok( () )
  }
}

/// Generalizes over row/column fragments in a sudoku
trait ThreeCellFragment {
  /// Returns the 3 cells inside the fragment
  fn cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a;

  /// Returns the cell indices outside the fragment, but that are still
  /// influenced by this fragment. (i.e., in the same block, and row or column,
  /// for row/column fragments, respectively)
  fn ext_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    self.ext_adjacent_cells( ).chain( self.ext_block_cells( ) )
  }

  /// Returns the cell indices outside the fragment, but inside the same row/column.
  ///   (for row/column fragments, respectively)
  fn ext_adjacent_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a;

  /// Returns the cell indices outside the fragment, but inside the same block.
  fn ext_block_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a;
}

/// A fragment of length 3 in a row.
struct RowFragment {
  /// Invariant: 0 <= y < 9
  y: usize,

  /// Invariant: 0 <= frag_x < 3
  frag_x: usize
}

/// A fragment of length 3 in a column.
struct ColFragment {
  /// Invariant: 0 <= x < 9
  x: usize,

  /// Invariant: 0 <= frag_y < 3
  frag_y: usize
}

impl ThreeCellFragment for RowFragment {
  fn cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    (0..3).map( |x| self.y * 9 + self.frag_x * 3 + x )
  }

  fn ext_adjacent_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    (3..9).map( |x| self.y * 9 + ( self.frag_x * 3 + x ) % 9 )
  }

  fn ext_block_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    // the y of the top row in *the block*
    let base_y = self.y - self.y % 3;
    (1..3).flat_map(
      move |y| {
        let other_row_y = base_y + ( self.y + y ) % 3;
        (0..3).map( move |x| other_row_y * 9 + ( self.frag_x * 3 + x ) )
      }
    )
  }
}

impl ThreeCellFragment for ColFragment {
  fn cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    (0..3).map( |y| ( self.frag_y * 3 + y ) * 9 + self.x )
  }

  fn ext_adjacent_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    (3..9).map( |y| ( ( self.frag_y * 3 + y ) % 9 ) * 9 + self.x )
  }

  fn ext_block_cells< 'a >( &'a self ) -> impl Iterator< Item = usize > + 'a {
    // the x of the first column in *the block*
    let base_x = self.x - self.x % 3;
    (1..3).flat_map(
      move |x| {
        let other_col_x = base_x + ( self.x + x ) % 3;
        (0..3).map( move |y| ( self.frag_y * 3 + y ) * 9 + other_col_x )
      }
    )
  }
}

type HasProgressed = bool;

/// Find values that necessarily exist in a 3-cell fragment, and exclude
/// those from the fragment in the same block and row/col (for row/col
/// fragments, respectively).
fn frag_exclusions< F: ThreeCellFragment >(
  sudoku: &mut Sudoku,
  frag: &F
) -> Result< HasProgressed, Error > {
  // A bitmask, set for values that must be in the fragment.
  let frag_necessary = sudoku.frag_necessary( frag );

  // If more than 3 values is necessary in a 3-cell fragment, surely it's
  // unsolvable.
  if frag_necessary.count_ones( ) > 3 {
    return Err( Error::Unsolvable );
  }

  let mut has_progressed = false;

  // Iterate over the cells influenced by our fragment
  //   (i.e., in the same block and row/column)
  for cell_idx in frag.ext_cells( ) {
    // The values that are necessary in our fragment, are impossible in
    // its influenced cells. So remove those options.

    if sudoku.0[ cell_idx ].0 & frag_necessary != 0 {
      has_progressed = true;
    }
    sudoku.0[ cell_idx ].0 &= !frag_necessary;
  }

  Ok( has_progressed )
}

/// Perform a single iteration of excluding outside all 3-cell row/col fragments
fn exclusion_step( sudoku: &mut Sudoku ) -> Result< HasProgressed, Error > {
  let mut has_progressed = false;

  for y in 0..9 {
    for frag_x in 0..3 {
      has_progressed = frag_exclusions( sudoku, &RowFragment { y, frag_x } )? || has_progressed;
    }
  }

  for x in 0..9 {
    for frag_y in 0..3 {
      has_progressed = frag_exclusions( sudoku, &ColFragment { x, frag_y } )? || has_progressed;
    }
  }

  Ok( has_progressed )
}

fn solve( sudoku: &mut Sudoku ) -> Result< (), Error > {
  // Keeps going as long as we made progress
  while exclusion_step( sudoku )? {
    println!( "== STEP" );
    println!( "{}", sudoku );
    println!( );
  }
  Ok( () )
}

#[cfg(test)]
mod tests;
