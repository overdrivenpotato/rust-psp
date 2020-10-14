use crate::sprite::Vertex;
use crate::Align4;
use crate::BLOCK_SIZE;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Gameboard {
    blocks: [Option<u32>; 200],
    width: usize,
    height: usize,
    block_spawn_loc: (usize, usize),
    gameboard_offset: (usize, usize)
}

impl Gameboard {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            blocks: [None; 200],
            width: 10,
            height: 20,
            block_spawn_loc: (10 / 2, 0),
            gameboard_offset: (x, y)
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

    fn get_content(&self, x: usize, y: usize) -> Option<u32> {
        self.blocks[self.point_to_index(x, y)?]
    }

    fn set_content(&mut self, x: usize, y: usize, content: Option<u32>) -> Result<(), ()>{
        self.blocks[self.point_to_index(x, y).ok_or(())?] = content;
        Ok(())
    }

    fn are_locs_empty(&self, locs: Vec<(usize, usize)>) -> bool {
        for point in locs {
            if self.get_content(point.0, point.1).is_some() {
                return false
            }
        }
        true
    }

    fn remove_completed_rows(&mut self) -> Result<usize, ()> {
        let row_indices = self.get_completed_row_indices();
        self.remove_rows(&row_indices)?;
        Ok(row_indices.len())
    }

    // do we need this? seems pointless
    //fn empty(&mut self) {
        //self = Self::new();
    //}
    
    fn get_spawn_loc(&self) -> (usize, usize) {
        self.block_spawn_loc
    }

    fn is_row_completed(&self, row_index: usize) -> bool {
        for x in 0..self.width {
            if self.get_content(x, row_index).is_none() {
                return false
            }
        }
        true
    }

    fn get_completed_row_indices(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for y in 0..self.height {
            if self.is_row_completed(y) {
                ret.push(y);
            }
        }
        ret
    }

    fn remove_row(&mut self, row_index: usize) -> Result<(), ()> {
        for y in (1..row_index).rev() {
           self.copy_row_into_row(y-1, y)?; 
        }
        self.fill_row(0, None)?;
        Ok(())
    }

    fn remove_rows(&mut self, row_indices: &Vec<usize>) -> Result<(), ()> {
        for index in row_indices {
            self.remove_row(*index)?
        }
        Ok(())
    }

    pub fn fill_row(&mut self, row_index: usize, content: Option<u32>) -> Result<(), ()> {
        for x in 0..self.width {
            self.set_content(x, row_index, content.clone())?;
        }
        Ok(())
    }

    fn copy_row_into_row(&mut self, src_row_index: usize, dst_row_index: usize) -> Result<(), ()>{
        for x in 0..self.width {
            self.set_content(x, dst_row_index, self.get_content(x, src_row_index))?
        }
        Ok(())
    }

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
                    x: ((x+self.gameboard_offset.0) as u32 * BLOCK_SIZE) as f32,
                    y: ((y+self.gameboard_offset.1) as u32 * BLOCK_SIZE) as f32,
                    z: 0.0,
                });
                ret[index+1] = Align4(Vertex {
                    u: BLOCK_SIZE as f32,
                    v: BLOCK_SIZE as f32,
                    color,
                    x: ((x+self.gameboard_offset.0) as u32 * BLOCK_SIZE) as f32 + BLOCK_SIZE as f32,
                    y: ((y+self.gameboard_offset.1) as u32 * BLOCK_SIZE) as f32 + BLOCK_SIZE as f32,
                    z: 0.0,
                });
            }
        }
        ret
    }
}
