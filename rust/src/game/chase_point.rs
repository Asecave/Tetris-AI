use macroquad::{color::{BLACK, BLUE, GREEN, RED, SKYBLUE}, shapes::{draw_circle, draw_rectangle}};
use rand::{thread_rng, Rng};

use crate::genome::Genome;

use super::Game;

#[derive(Clone, Default)]
pub struct ChasePoint {
    pub point_x: f32,
    pub point_y: f32,
    pub player_x: f32,
    pub player_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub total_distance: f32,
    pub frames_on_point: u32,
}

impl ChasePoint {

    fn move_x(&mut self, amount: f32) {
        self.vel_x += f32::min(1.0, f32::max(-1.0, amount)) * 0.01;
    }

    fn move_y(&mut self, amount: f32) {
        self.vel_y += f32::min(1.0, f32::max(-1.0, amount)) * 0.01;
    }

    fn target_distance2(&self) -> f32 {
        (self.point_x - self.player_x) * (self.point_x - self.player_x) + 
        (self.point_y - self.player_y) * (self.point_y - self.player_y) 
    }
}

impl Game for ChasePoint {
    
    fn new() -> Self {
        let mut game = Self::default();
        let angle = thread_rng().gen_range(0.0..2.0 * std::f32::consts::PI);
        let distance = 1.0;
        let pos_x = distance * f32::cos(angle);
        let pos_y = distance * f32::sin(angle);
        game.point_x = pos_x;
        game.point_y = pos_y;
        game
    }
    
    fn base_genome_io() -> (u32, u32) {
        (4, 2)
    }

    fn set_input_node_values(&self, genome: &mut Genome) {
        genome.graph.node_weight_mut(genome.input_nodes[0]).unwrap().value = self.point_x;
        genome.graph.node_weight_mut(genome.input_nodes[1]).unwrap().value = self.point_y;
        genome.graph.node_weight_mut(genome.input_nodes[2]).unwrap().value = self.player_x;
        genome.graph.node_weight_mut(genome.input_nodes[3]).unwrap().value = self.player_y;
    }

    fn apply_outputs(&mut self, genome: &Genome) {
        self.move_x(genome.graph.node_weight(genome.output_nodes[0]).unwrap().value);
        self.move_y(genome.graph.node_weight(genome.output_nodes[1]).unwrap().value);
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
    
    fn fitness_function(&mut self, fitness: f32) -> f32 {
        let mut new_fitness = 1.0 / (self.target_distance2() + 1.0);
        if self.total_distance > 5.0 {
            new_fitness /= self.total_distance;
        }
        if new_fitness > 0.9999 {
            self.frames_on_point += 1;
        } else {
            self.frames_on_point = 0;
        }
        if self.frames_on_point > 100 {
            new_fitness += 10.0;
        }
        new_fitness + fitness
    }

    fn draw_static(&self, x: f32, y: f32, scale: f32) {
        draw_rectangle(x - scale, y - scale, scale * 2.0, scale * 2.0, BLACK);
    }

    fn draw_dynamic(&self, x: f32, y: f32, scale: f32) {
        let mut color = RED;
        color.a = 0.25;
        draw_rectangle(self.point_x * scale + (x - scale * 0.025), self.point_y * scale + (y - scale * 0.025), scale * 0.05, scale * 0.05, color);
        let mut color = if self.frames_on_point > 100 {BLUE} else {SKYBLUE};
        color.a = 0.25; //f32::max(0.0, 1.0 - self.target_distance2() * 100.0);
        draw_circle(self.player_x * scale + x, self.player_y * scale + y, scale * 0.02, color);
    }

    fn draw_best(&self, x: f32, y: f32, scale: f32) {
        self.draw_dynamic(x, y, scale);
        draw_circle(self.player_x * scale + x, self.player_y * scale + y, scale * 0.02, GREEN);
    }
}