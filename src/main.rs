use std::fmt::{Display, Debug};

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

  /// Returns `true` iff the provided value is possible in this cell
  pub fn is_possible( &self, v: usize ) -> bool {
    debug_assert!( 1 <= v && v <= 9 );
    ( ( self.0 >> ( v - 1 ) ) & 1 ) != 0
  }
}

impl Debug for SudokuCell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!( f, "{{" )?;
    for i in 1..=9 {
      if self.is_possible( i ) {
        write!( f, "{},", i )?;
      }
    }
    write!( f, "}}" )
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
}

impl From< [u8; 9*9] > for Sudoku {
  fn from( cells: [u8; 9*9] ) -> Sudoku {
    // We construct a sudoku from the cells. 0 represents that everything is
    // possible
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
        if x == 2 || x == 5 {
          write!( f, " " )?;
        }
      }
      writeln!( f )?;
      if y == 2 || y == 5 {
        writeln!( f )?;
      }
    }
    Ok( () )
  }
}

fn solve( sudoku: &mut Sudoku ) -> Result< (), Error > {
  // Keeps going as long as we made progress
  while row_necessary_exclusions( sudoku )?
          || col_necessary_exclusions( sudoku )? {
    println!( "== STEP" );
    println!( "{}", sudoku );
    println!( );
  }
  Ok( () )
}

/// Returns that bitmask of values that necessarily exist in the row fragment.
/// 
/// Precondition: 0 <= y < 9, 0 <= frag_x < 3
fn find_row_necessary( sudoku: &Sudoku, y: usize, frag_x: usize ) -> usize {
  // values that are possible outside our fragment
  let mut row_ext_possible   = 0b000_000_000;
  let mut block_ext_possible = 0b000_000_000;

  // check the entire row
  for x in 0..9 {
    if x < frag_x * 3 || x >= frag_x * 3 + 3 { // in *other* blocks
      row_ext_possible |= sudoku.0[ y * 9 + x ].0;
    }
  }

  // check the other 2 rows in the block
  for block_y in (y/3)*3..((y/3)*3 + 3) {
    if block_y != y { // *other* rows
      for x in 0..3 {
        block_ext_possible |= sudoku.0[ block_y * 9 + frag_x * 3 + x ].0;
      }
    }
  }

  // the things that are impossible outside our fragment, surely have to be
  // inside our fragment.
  let mut frag_necessary = ( !row_ext_possible | !block_ext_possible ) & 0b111_111_111;
  for x in 0..3 {
    let i = y * 9 + frag_x * 3 + x;
    if let Some( v ) = sudoku.0[ i ].solution( ) {
      frag_necessary |= 1 << ( v - 1 );
    }
  }

  frag_necessary
}

/// Find values that necessarily exist in a particular row fragment, and exclude
/// those from the fragment in the same row and block.
fn row_necessary_exclusions( sudoku: &mut Sudoku ) -> Result< bool, Error > {
  let mut has_progressed = false;
  for y in 0..9 {
    // there are 3 "row fragments" of length 3 in a row.
    for frag_x in 0..3 {
      let frag_necessary = find_row_necessary( sudoku, y, frag_x );

      if frag_necessary.count_ones( ) > 3 {
        return Err( Error::Unsolvable );
      }

      for x in 0..9 {
        if x < frag_x * 3 || x >= frag_x * 3 + 3 { // in *other* blocks
          if sudoku.0[ y * 9 + x ].0 & frag_necessary != 0 {
            // we'll remove an option, hence we progressed
            has_progressed = true;
          }
          sudoku.0[ y * 9 + x ].0 &= !frag_necessary;
        }
      }

      for block_y in (y/3)*3..((y/3)*3 + 3) {
        if block_y != y { // *other* rows
          for x in 0..3 {
            let i = block_y * 9 + frag_x * 3 + x;
            if sudoku.0[ i ].0 & frag_necessary != 0 {
              // we'll remove an option, hence we progressed
              has_progressed = true;
            }
            sudoku.0[ i ].0 &= !frag_necessary;
          }
        }
      }
    }
  }

  Ok( has_progressed )
}

/// Returns that bitmask of values that necessarily exist in the column
/// fragment.
/// 
/// Precondition: 0 <= x < 9, 0 <= frag_y < 3
fn find_col_necessary( sudoku: &mut Sudoku, x: usize, frag_y: usize ) -> usize {
  // values that are possible outside our fragment
  let mut col_ext_possible   = 0b000_000_000;
  let mut block_ext_possible = 0b000_000_000;

  // check the entire column
  for y in 0..9 {
    if y < frag_y * 3 || y >= frag_y * 3 + 3 { // in *other* blocks
      col_ext_possible |= sudoku.0[ y * 9 + x ].0;
    }
  }

  // check the other 2 columns in the block
  for block_x in (x/3)*3..((x/3)*3 + 3) {
    if block_x != x { // *other* columns
      for y in 0..3 {
        block_ext_possible |= sudoku.0[ ( frag_y * 3 + y ) * 9 + block_x ].0;
      }
    }
  }

  // the things that are impossible outside our fragment, surely have to be
  // inside our fragment.
  let mut frag_necessary = ( !col_ext_possible | !block_ext_possible ) & 0b111_111_111;
  for y in 0..3 {
    let i = ( frag_y * 3 + y ) * 9 + x;
    if let Some( v ) = sudoku.0[ i ].solution( ) {
      frag_necessary |= 1 << ( v - 1 );
    }
  }
  frag_necessary
}

/// Find values that necessarily exist in a particular column fragment, and
/// exclude those from the fragment in the same column and block.
fn col_necessary_exclusions( sudoku: &mut Sudoku ) -> Result< bool, Error > {
  let mut has_progressed = false;

  for x in 0..9 {
    // there are 3 "column fragments" of length 3 in a column.
    for frag_y in 0..3 {
      let frag_necessary = find_col_necessary( sudoku, x, frag_y );

      if frag_necessary.count_ones( ) > 3 {
        return Err( Error::Unsolvable );
      }

      for y in 0..9 {
        if y < frag_y * 3 || y >= frag_y * 3 + 3 { // in *other* blocks
          if sudoku.0[ y * 9 + x ].0 & frag_necessary != 0 {
            // we'll remove an option, hence we progressed
            has_progressed = true;
          }
          sudoku.0[ y * 9 + x ].0 &= !frag_necessary;
        }
      }

      for block_x in (x/3)*3..((x/3)*3 + 3) {
        if block_x != x { // *other* columns
          for y in 0..3 {
            let i = ( frag_y * 3 + y ) * 9 + block_x;
            if sudoku.0[ i ].0 & frag_necessary != 0 {
              // we'll remove an option, hence we progressed
              has_progressed = true;
            }
            sudoku.0[ i ].0 &= !frag_necessary;
          }
        }
      }
    }
  }

  Ok( has_progressed )
}

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
  
  // Keeps going as long as we made progress
  if let Ok( () ) = solve( &mut sudoku ) {
    if sudoku.is_solved( ) {
      println!( "== SOLVED" );
    } else {
      println!( "== SOLVING FAILED" );
    }
  } else {
    println!( "== UNSOLVABLE" );
  }
}
