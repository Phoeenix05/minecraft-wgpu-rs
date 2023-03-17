use std::collections::HashMap;

use crate::world::block::Block;
use crate::world::block_constants::*;
use crate::gpu_types::Vertex;

use super::BlockTrait;

#[derive(Debug)]
pub struct Chunk {
    sub_chunks: HashMap<(i32, i32, i32), SubChunk>,
    mesh: Vec<Vertex>,
}

impl Chunk {
    pub fn create_mesh(&mut self) {
        // Zero / refresh the mesh
        self.mesh = Vec::new();

        // Iterate sub chunks and their meshes and append all vertices
        // to this chunk's mesh
        for (_, sub_chunk) in self.sub_chunks.iter() {
            for vertex in sub_chunk.mesh.iter() {
                // self.mesh.push(vertex.as_);
            }
        }
    }
}

#[derive(Debug)]
pub struct SubChunk {
    blocks: [[[Block; 16]; 16]; 16],
    mesh: Vec<[Vertex; 4]>,
}

impl SubChunk {
    pub fn create_mesh(&mut self) {
        // Zero / refresh the mesh
        self.mesh = Vec::new();

        // self.blocks.map(|y| y).map(|z| z)
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    // self.mesh.append(&mut BLOCK_VERTICES.to_vec());
                    // if self.blocks[i + 1][j][k].is_transparent() {
                    //     // self.mesh.append(BLOCK_TOP_VERTICES);
                    // }
                    // if self.blocks[i - 1][j][k].is_transparent() {}
                    // if self.blocks[i][j][k].is_transparent() {}
                    // if self.blocks[i][j][k].is_transparent() {}
                    // if self.blocks[i][j][k].is_transparent() {}
                    // if self.blocks[i][j][k].is_transparent() {}
                }
            }
        }
    }
}
