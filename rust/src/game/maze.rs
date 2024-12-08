use macroquad::{color::{BLACK, GOLD, GREEN, RED, SKYBLUE}, shapes::{draw_circle, draw_circle_lines, draw_rectangle}};

use crate::genome::Genome;

use super::Game;


#[derive(Clone)]
pub struct Maze {
    board: [[bool; 10]; 10],
    player_x: f32,
    player_y: f32,
    dead: bool,
    coins: Vec<(f32, f32, bool)>,
}

impl Game for Maze {

    fn new() -> Self {
        Maze {
            board: [
                [false, false, false, false, false, false, false, false, false, false],
                [true , true , true , true , true , false, true , true , true , true ],
                [false, false, false, false, false, false, false, false, false, false],
                [false, true , true , true , true , true , true , true , true , true ],
                [false, false, false, false, false, false, false, false, false, false],
                [true , true , true , true , false, true , true , true , true , true ],
                [false, false, false, false, false, false, false, false, false, false],
                [true , true , true , false, true , true , true , true , true , true ],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
            ],
            player_x: 0.0,
            player_y: 0.95,
            dead: false,
            coins: vec![
                ((3.0 + 0.5) / 10.0 * 2.0 - 1.0, (7.0 + 0.5) / 10.0 * 2.0 - 1.0, false),
                ((4.0 + 0.5) / 10.0 * 2.0 - 1.0, (5.0 + 0.5) / 10.0 * 2.0 - 1.0, false),
                ((0.0 + 0.5) / 10.0 * 2.0 - 1.0, (3.0 + 0.5) / 10.0 * 2.0 - 1.0, false),
                ((5.0 + 0.5) / 10.0 * 2.0 - 1.0, (1.0 + 0.5) / 10.0 * 2.0 - 1.0, false),
            ],
        }
    }

    fn base_genome_io() -> (u32, u32) {
        (1, 2)
    }

    fn set_input_node_values(&self, genome: &mut Genome) {
        genome.graph.node_weight_mut(genome.input_nodes[0]).unwrap().value = self.player_y;
    }

    fn apply_outputs(&mut self, genome: &Genome) {
        if self.dead {
            return;
        }
        self.player_x += genome.graph.node_weight(genome.output_nodes[0]).unwrap().value * 0.05;
        self.player_y += genome.graph.node_weight(genome.output_nodes[1]).unwrap().value * 0.05;
    }

    fn update(&mut self) {
        if self.player_x < -1.0 {
            self.player_x = -1.0;
        }
        if self.player_y < -1.0 {
            self.player_y = -1.0;
        }
        if self.player_x > 1.0 {
            self.player_x = 1.0;
        }
        if self.player_y > 1.0 {
            self.player_y = 1.0;
        }

        if self.board[u32::min(9, ((self.player_y + 1.0) / 2.0 * self.board.len() as f32) as u32) as usize][u32::min(9, ((self.player_x + 1.0) / 2.0 * self.board.len() as f32) as u32) as usize] {
            self.dead = true;
        }
    }
    
    fn fitness_function(&mut self, fitness: f32) -> f32 {
        if self.dead {
            fitness
        } else {

            let mut add_fitness = 0.0;
            let mut coin_dst2 = 0.0;

            for (coin_x, coin_y, collected) in self.coins.iter_mut() {
                if *collected {
                    continue;
                }
                let current_coin_dst2 = (*coin_x - self.player_x) * (*coin_x - self.player_x) + (*coin_y - self.player_y) * (*coin_y - self.player_y);
                if current_coin_dst2 < 0.001 {
                    *collected = true;
                    add_fitness += 10000.0;
                }
                coin_dst2 = current_coin_dst2;
                break;
            }

            fitness + add_fitness + (1.0 / (coin_dst2 + 1.0))
        }
    }

    fn draw_static(&self, x: f32, y: f32, scale: f32) {
        draw_rectangle(x - scale, y - scale, scale * 2.0, scale * 2.0, BLACK);
        for (by, col) in self.board.iter().enumerate() {
            for (bx, tile) in col.iter().enumerate() {
                if !tile {
                    continue;
                }
                let pos_x = (bx as f32 / self.board.len() as f32 * 2.0 - 1.0) * scale + x;
                let pos_y = (by as f32 / self.board.len() as f32 * 2.0 - 1.0) * scale + y;
                let box_size = 2.0 / self.board.len() as f32 * scale;
                draw_rectangle(pos_x, pos_y, box_size, box_size, RED);
            }
        }
    }

    fn draw_dynamic(&self, x: f32, y: f32, scale: f32) {
        let mut color = SKYBLUE;
        if self.dead {
            color.a = 0.5;
        }
        draw_circle(self.player_x * scale + x, self.player_y * scale + y, scale * 0.02, color);
    }

    fn draw_best(&self, x: f32, y: f32, scale: f32) {
        draw_circle(self.player_x * scale + x, self.player_y * scale + y, scale * 0.02, GREEN);

        for &(coin_x, coin_y, collected) in self.coins.iter() {
            if collected {
                continue;
            }
            draw_circle_lines(coin_x * scale + x, coin_y * scale + y, scale * 0.025, 1.0, GOLD);
        }
    }
}