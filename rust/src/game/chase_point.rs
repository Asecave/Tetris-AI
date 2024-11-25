use rand::{thread_rng, Rng};

use crate::genome::{ActivationFunction, Genome, Node};

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
        let pos_x = 0.9;//if thread_rng().gen_bool(0.5) {-0.9} else {0.9};
        let pos_y = 0.9;//if thread_rng().gen_bool(0.5) {-0.9} else {0.9};
        // let angle = thread_rng().gen_range(0.0..2.0 * std::f32::consts::PI);
        // let distance = 1.0;
        // let pos_x = distance * f32::cos(angle);
        // let pos_y = distance * f32::sin(angle);
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
        self.vel_x += f32::min(1.0, f32::max(-1.0, amount)) * 0.0002;
    }

    pub fn move_y(&mut self, amount: f32) {
        self.vel_y += f32::min(1.0, f32::max(-1.0, amount)) * 0.0002;
    }

    pub fn target_distance2(&self) -> f32 {
        (self.point_x - self.player_x) * (self.point_x - self.player_x) + 
        (self.point_y - self.player_y) * (self.point_y - self.player_y) 
    }
}

impl Game for ChasePoint {
    fn create_base_genome(&self) -> Genome {
        let mut nodes: Vec<Node> = Vec::with_capacity(6);

        for _ in 0..nodes.capacity() {
            let mut node = Node::new();
            node.activation_function = ActivationFunction::Tanh;
            nodes.push(node);
        }

        let output_nodes = vec![4, 5];

        return Genome::new(nodes, output_nodes);
    }

    fn set_input_node_values(&self, genome: &mut Genome) {
        genome.set_node_value(0, self.point_x);
        genome.set_node_value(1, self.point_y);
        genome.set_node_value(2, self.player_x);
        genome.set_node_value(3, self.player_y);
    }
    
    fn update(&mut self) {
        self.player_x += self.vel_x;
        self.player_y += self.vel_y;

        self.total_distance += f32::abs(self.vel_x);
        self.total_distance += f32::abs(self.vel_y);

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