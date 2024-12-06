use petgraph::visit::IntoNodeReferences;

use crate::{config, game::Game, genome::{Genome, NodeType}};

#[derive(Clone)]
pub struct Agent {
    pub genome: Genome,
    pub game: config::GAME,
    pub fitness: f32,
}

impl Agent {

    pub fn new() -> Self {
        let game = config::GAME::new();
        Self {
            genome: game.create_base_genome(),
            game,
            fitness: 0.0,
        }
    }

    pub fn play(&mut self) {

        self.game.set_input_node_values(&mut self.genome);
        self.genome.traverse();
        self.game.apply_outputs(&self.genome);
        self.game.update();

        self.fitness = self.game.fitness_function(self.fitness);
    }

    pub fn clone_and_keep_io_nodes(&self) -> Self {
        let mut a = self.clone();
        let number_of_inputs = a.genome.input_nodes.len();
        let number_of_outputs = a.genome.output_nodes.len();
        a.genome.input_nodes = Vec::new();
        a.genome.output_nodes = Vec::new();
        while a.genome.input_nodes.len() < number_of_inputs {
            for (index, node) in a.genome.graph.node_references() {
                if node.node_type == NodeType::Input(a.genome.input_nodes.len() as u32) {
                    a.genome.input_nodes.push(index);
                }
            }
        }
        while a.genome.output_nodes.len() < number_of_outputs {
            for (index, node) in a.genome.graph.node_references() {
                if node.node_type == NodeType::Output(a.genome.output_nodes.len() as u32) {
                    a.genome.output_nodes.push(index);
                }
            }
        }
        a
    }
}
