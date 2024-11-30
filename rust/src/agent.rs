use petgraph::visit::IntoNodeReferences;

use crate::{game::{chase_point::ChasePoint, Game}, genome::{Genome, NodeType}};

#[derive(Clone)]
pub struct Agent {
    pub genome: Genome,
    pub game: ChasePoint,
    pub fitness: f32,
    pub frames_on_point: u32,
}

impl Agent {

    pub fn new() -> Self {
        let game = ChasePoint::new();
        Self {
            genome: game.create_base_genome(),
            game,
            fitness: 0.0,
            frames_on_point: 0,
        }
    }

    pub fn play(&mut self) {

        self.game.set_input_node_values(&mut self.genome);
        self.genome.traverse();
        self.game.move_x(self.genome.graph.node_weight(self.genome.output_nodes[0]).unwrap().value);
        self.game.move_y(self.genome.graph.node_weight(self.genome.output_nodes[1]).unwrap().value);
        self.game.update();

        self.fitness = self.fitness_function();
    }

    fn fitness_function(&mut self) -> f32 {
        let mut fitness = 1.0 / (self.game.target_distance2() + 1.0);
        // fitness *= f32::powf(0.5, f32::abs(self.game.vel_x) + f32::abs(self.game.vel_y));
        if self.game.total_distance > 3.0 {
            fitness /= self.game.total_distance;
        }
        if fitness > 0.9999 {
            self.frames_on_point += 1;
        } else {
            self.frames_on_point = 0;
        }
        if self.frames_on_point > 100 {
            fitness += 10.0;
        }
        self.fitness + fitness
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
