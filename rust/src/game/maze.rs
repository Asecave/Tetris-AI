use petgraph::graph::DiGraph;

use crate::genome::Genome;

use super::Game;


#[derive(Clone)]
pub struct Maze;

impl Game for Maze {

    fn new() -> Self {
        Maze
    }

    fn create_base_genome(&self) -> Genome {
        Genome::new(DiGraph::new(), vec![], vec![])
    }

    fn set_input_node_values(&self, genome: &mut Genome) {
        
    }

    fn apply_outputs(&mut self, genome: &Genome) {
        
    }

    fn update(&mut self) {
        
    }
    
    fn fitness_function(&mut self, fitness: f32) -> f32 {
        0.0
    }

    fn draw_dynamic(&self, x: f32, y: f32, scale: f32) {
        
    }

    fn draw_static(&self, x: f32, y: f32, scale: f32) {
        
    }

    fn draw_best(&self, x: f32, y: f32, scale: f32) {
        
    }
}