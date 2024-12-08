use crate::game::maze::Maze;

pub const NUM_AGENTS: usize = 1000;
pub const FRAMES_PER_GEN: u32 = 1000;
pub const MAX_NODES: u32 = 8;
pub const MAX_EDGES: u32 = 8;
pub const USE_PARALLELISM: bool = true;
pub type GAME = Maze;