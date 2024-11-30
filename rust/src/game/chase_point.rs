use petgraph::graph::{DiGraph, NodeIndex};
use rand::{thread_rng, Rng};

use crate::genome::{Genome, Node, NodeType};

use super::Game;

#[derive(Clone)]
pub struct ChasePoint {
    pub point_x: f32,
    pub point_y: f32,
    pub player_x: f32,
    pub player_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub total_distance: f32,
}

impl ChasePoint {
    pub fn new() -> Self {
        // let pos_x = 0.9;//if thread_rng().gen_bool(0.5) {-0.9} else {0.9};
        // let pos_y = 0.9;//if thread_rng().gen_bool(0.5) {-0.9} else {0.9};
        let angle = thread_rng().gen_range(0.0..2.0 * std::f32::consts::PI);
        let distance = 1.0;
        let pos_x = distance * f32::cos(angle);
        let pos_y = distance * f32::sin(angle);
        Self {
            point_x: 0.0,
            point_y: 0.0,
            player_x: pos_x,
            player_y: pos_y,
            vel_x: 0.0,
            vel_y: 0.0,
            total_distance: 0.0,
        }
    }

    pub fn move_x(&mut self, amount: f32) {
        self.vel_x += f32::min(1.0, f32::max(-1.0, amount)) * 0.01;
    }

    pub fn move_y(&mut self, amount: f32) {
        self.vel_y += f32::min(1.0, f32::max(-1.0, amount)) * 0.01;
    }

    pub fn target_distance2(&self) -> f32 {
        (self.point_x - self.player_x) * (self.point_x - self.player_x) + 
        (self.point_y - self.player_y) * (self.point_y - self.player_y) 
    }
}

impl Game for ChasePoint {
    fn create_base_genome(&self) -> Genome {
        let mut graph: petgraph::Graph<Node, f32> = DiGraph::new();
        let mut input_nodes: Vec<NodeIndex> = Vec::new();
        let mut output_nodes: Vec<NodeIndex> = Vec::new();

        let mut input_index = 0;
        let mut output_index = 0;
        for i in 0..6 {
            if i < 4 {
                let index = graph.add_node(Node {
                    value: 0.0,
                    bias: 0.0,
                    node_type: NodeType::Input(input_index),
                    layer: 0,
                });
                input_nodes.push(index);
                input_index += 1;
            } else {
                let index = graph.add_node(Node {
                    value: 0.0,
                    bias: 0.0,
                    node_type: NodeType::Output(output_index),
                    layer: 1,
                });
                output_nodes.push(index);
                output_index += 1;
            }
        }

        return Genome::new(graph, input_nodes, output_nodes);
    }

    fn set_input_node_values(&self, genome: &mut Genome) {
        genome.graph.node_weight_mut(genome.input_nodes[0]).unwrap().value = self.point_x;
        genome.graph.node_weight_mut(genome.input_nodes[1]).unwrap().value = self.point_y;
        genome.graph.node_weight_mut(genome.input_nodes[2]).unwrap().value = self.player_x;
        genome.graph.node_weight_mut(genome.input_nodes[3]).unwrap().value = self.player_y;
    }
    
    fn update(&mut self) {
        self.player_x += self.vel_x;
        self.player_y += self.vel_y;

        self.total_distance += f32::abs(self.vel_x);
        self.total_distance += f32::abs(self.vel_y);

        self.vel_x *= 0.95;
        self.vel_y *= 0.95;

        if self.player_x > 1.0 {
            self.player_x = 1.0;
        }
        if self.player_x < -1.0 {
            self.player_x = -1.0;
        }
        if self.player_y > 1.0 {
            self.player_y = 1.0;
        }
        if self.player_y < -1.0 {
            self.player_y = -1.0;
        }
    }
}