use crate::graphics::sprite::Vertex;
use crate::graphics::Align4;
use crate::BLOCK_SIZE;
use crate::{GAMEBOARD_OFFSET, GAMEBOARD_WIDTH, GAMEBOARD_HEIGHT};
use alloc::vec::Vec;

/// The playing field of tetris.
#[derive(Debug)]
pub struct Gameboard {
    blocks: [Option<u32>; 200],
    width: usize,
    height: usize,
    block_spawn_loc: (usize, usize),
}

impl Gameboard {
    /// Creates a new `Gameboard`. 
    pub fn new() -> Self {
        Self {
            blocks: [None; 200],
            width: GAMEBOARD_WIDTH,
            height: GAMEBOARD_HEIGHT,
            block_spawn_loc: (GAMEBOARD_WIDTH / 2, 1),
        }
    }

    #[inline]
    const fn point_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some((x + y * self.width) as usize);
        }
        None
    }

    #[inline]
    const fn index_to_point(&self, index: usize) -> (usize, usize) {
        let y = index / self.width;
        let x = index % self.width;
        (x, y)
    }

    /// Gets the colour of the block at position (x, y), 
    /// or None if the position is empty.
    ///
    /// # Parameters
    ///
    /// `x`: Horizontal position within the gameboard
    /// `y`: Vertical position within the gameboard
    ///
    /// # Return Value
    /// The colour of the block at position (x, y), 
    /// or None if the position is empty.
    pub fn get_content(&self, x: usize, y: usize) -> Option<u32> {
        self.blocks[self.point_to_index(x, y)?]
    }

    /// Sets the colour of the block at position (x, y), 
    /// or None if the position is empty.
    ///
    /// # Parameters
    ///
    /// - `x`: Horizontal position within the gameboard
    /// - `y`: Vertical position within the gameboard
    /// - `content`: Colour of the block at position (x, y), or None if the position is 
    /// empty.
    ///
    /// # Return Value
    ///
    /// Ok(()) if the position is valid, Err(()) otherwise.
    pub fn set_content(&mut self, x: usize, y: usize, content: Option<u32>) -> Result<(), ()>{
        self.blocks[self.point_to_index(x, y).ok_or(())?] = content;
        Ok(())
    }

    /// Checks if the given block positions are empty.
    ///
    /// # Parameters
    ///
    /// - `locs`: `Vec` of position tuples to check.
    ///
    /// # Return Value
    /// 
    /// `true` if all block positions are empty, `false` otherwise.
    pub fn are_locs_empty(&self, locs: Vec<(usize, usize)>) -> bool {
        for point in locs {
            if self.get_content(point.0, point.1).is_some() {
                return false
            }
        }
        true
    }

    /// Removes all rows which are full along the horizontal axis.
    ///
    /// # Return Value
    ///
    /// The number of rows removed.
    pub fn remove_completed_rows(&mut self) -> usize {
        let row_indices = self.get_completed_row_indices();
        self.remove_rows(&row_indices).unwrap();
        row_indices.len()
    }

    /// Returns the position within the `Gameboard` at which new blocks are spawned.
    ///
    /// # Return Value
    ///
    /// The position within the `Gameboard` at which new blocks are spawned.
    pub fn get_spawn_loc(&self) -> (usize, usize) {
        (self.block_spawn_loc.0 + GAMEBOARD_OFFSET.0, self.block_spawn_loc.1 + GAMEBOARD_OFFSET.1)
    }


    /// Returns `true` if the given `row_index` is horizontally full.
    /// 
    /// # Parameters
    ///
    /// - `row_index`: The index of the row to check, from 0 to `GAMEBOARD_HEIGHT-1`
    ///
    /// # Return Value
    /// 
    /// `true` if the row is horizontally full, `false` otherwise.
    pub fn is_row_completed(&self, row_index: usize) -> bool {
        for x in 0..self.width {
            if self.get_content(x, row_index).is_none() {
                return false
            }
        }
        true
    }

    /// Returns all horizontally full rows within the Gameboard.
    ///
    /// # Return Value
    ///
    /// A `Vec` of rows which are horizontally full.
    pub fn get_completed_row_indices(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for y in 0..self.height {
            if self.is_row_completed(y) {
                ret.push(y);
            }
        }
        ret
    }

    /// Removes a row and moves down the rows above it to fill.
    ///
    /// # Parameters
    ///
    /// - `row_index`: The index of the row to remove.
    ///
    /// # Return Value
    ///
    /// Ok(()) if the row index is valid and the operation is successful, Err(()) 
    /// otherwise.
    pub fn remove_row(&mut self, row_index: usize) -> Result<(), ()> {
        for y in (1..=row_index).rev() {
           self.copy_row_into_row(y-1, y)?; 
        }
        self.fill_row(0, None)?;
        Ok(())
    }

    /// Removes multiple rows and moves down the rows above to fill.
    ///
    /// # Parameters
    ///
    /// - `row_indices`: The indices of the row to remove.
    ///
    /// # Return Value
    ///
    /// Ok(()) if the row indices are valid and the operation is successful, Err(()) 
    /// otherwise.
    pub fn remove_rows(&mut self, row_indices: &Vec<usize>) -> Result<(), ()> {
        for index in row_indices {
            self.remove_row(*index)?
        }
        Ok(())
    }

    /// Fills a row with blocks of the given colour, or empties it if `content` is None.
    ///
    /// # Parameters 
    ///
    /// - `row_index`: Index of the row to fill, from 0 to `GAMEBOARD_HEIGHT-1`.
    /// - `content`: Colour of the block to fill with, or None to empty.
    ///
    /// # Return value
    ///
    /// Ok(()) if row_index is valid, Err(()) otherwise.
    pub fn fill_row(&mut self, row_index: usize, content: Option<u32>) -> Result<(), ()> {
        for x in 0..self.width {
            self.set_content(x, row_index, content.clone())?;
        }
        Ok(())
    }

    /// Copies a row into another row. 
    /// Used to move rows above down when a row is completed.
    ///
    /// # Parameters: 
    ///
    /// - `src_row_index`: row index to copy from
    /// - `dst_row_index`: row index to copy to
    ///
    /// # Return Value
    ///
    /// Ok(()) if both indices are valid, Err(()) otherwise.
    pub fn copy_row_into_row(&mut self, src_row_index: usize, dst_row_index: usize) -> Result<(), ()>{
        for x in 0..self.width {
            self.set_content(x, dst_row_index, self.get_content(x, src_row_index))?
        }
        Ok(())
    }

    /// Returns a representation of the Gameboard as vertices which can be drawn.
    /// See `Sprite::Vertex` and `graphics::draw_vertices`
    ///
    /// # Return Value
    ///
    /// A representation of the Gameboard as vertices which can be drawn.
    pub fn as_vertices(&self) -> [Align4<Vertex>; 400] {
        let mut ret = [Align4(Vertex::default()); 400];
        for (index, block) in self.blocks.iter().enumerate() {
            let (x, y) = self.index_to_point(index);
            let index = index * 2;
            if block.is_some() {
                let color = block.unwrap();
                ret[index] = Align4(Vertex {
                    u: 0.0,
                    v: 0.0,
                    color,
                    x: ((x+GAMEBOARD_OFFSET.0) as u32 * BLOCK_SIZE) as f32,
                    y: ((y+GAMEBOARD_OFFSET.1) as u32 * BLOCK_SIZE) as f32,
                    z: 0.0,
                });
                ret[index+1] = Align4(Vertex {
                    u: BLOCK_SIZE as f32,
                    v: BLOCK_SIZE as f32,
                    color,
                    x: ((x+GAMEBOARD_OFFSET.0) as u32 * BLOCK_SIZE) as f32 + BLOCK_SIZE as f32,
                    y: ((y+GAMEBOARD_OFFSET.1) as u32 * BLOCK_SIZE) as f32 + BLOCK_SIZE as f32,
                    z: 0.0,
                });
            }
        }
        ret
    }
}
