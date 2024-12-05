use std::{collections::HashMap, sync::{Arc, Mutex}};

use macroquad::prelude::*;
use miniquad::window::set_window_size;
use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction::Outgoing};

use crate::{game::chase_point::ChasePoint, genome::Genome, UIShared, FRAMES_PER_GEN};

pub async fn open_ui(ui_shared: Arc<Mutex<UIShared>>) {

    set_window_size(1600, 1000);

    let mut previous_ui: Option<UIShared> = None;

    loop {

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(DARKGRAY);

        rect(screen_width() / 2.0 - 250.0, screen_height() / 2.0, 500.0, 400.0, LIGHTGRAY);

        if let Ok(ui) = ui_shared.try_lock() {
            previous_ui = Some((*ui).clone());
        }
        if let Some(ui) = previous_ui.clone() {
            let mut total_dst = 0.0;
            if let Some(agent) = &ui.agent {
                draw_genome(&agent.genome, screen_width() / 2.0, screen_height() / 2.0 + 200.0);
                draw_game(&agent.game, screen_width() / 2.0, screen_height() / 2.0 - 250.0);

                total_dst = agent.game.total_distance;
            }
            let mut info_text = String::new();

            info_text.push_str(format!("Generation: {}\n", &ui.generation).as_str());
            info_text.push_str(format!("Best fitness: {}\n", &ui.best_fitness).as_str());
            info_text.push_str(format!("Frame: {}/{}\n", &ui.current_frame, FRAMES_PER_GEN).as_str());
            info_text.push_str(format!("Total Distance: {}\n", total_dst).as_str());
            info_text.push_str(format!("Current fitness: {}\n", &ui.current_fitness).as_str());
            info_text.push_str(format!("Last evaluation time: {}ms\n", &ui.last_evaluation_time).as_str());
            info_text.push_str(format!("Last selection & mutation time: {}ms\n", &ui.last_selection_mutation_time).as_str());
            info_text.push_str(format!("Sleep time: {}ms\n", &ui.sleep_time).as_str());

            let mut text_row_offset = 0.0;
            for line in info_text.split("\n") {
                draw_text(line, 50.0, text_row_offset + 50.0, 30.0, BLACK);
                text_row_offset += 30.0;
            }

            // Input
            if is_key_pressed(KeyCode::Down) && ui.sleep_time != 0 {
                let mut ui = ui_shared.lock().unwrap();
                ui.sleep_time -= 10;
            }
            if is_key_pressed(KeyCode::Up) {
                let mut ui = ui_shared.lock().unwrap();
                ui.sleep_time += 10;
            }
        }

        next_frame().await
    }
}

fn draw_game(game: &ChasePoint, x: f32, y: f32) {
    draw_rectangle(x - 200.0, y - 200.0, 400.0, 400.0, BLACK);
    draw_rectangle(game.point_x * 200.0 + (x - 5.0), game.point_y * 200.0 + (y - 5.0), 10.0, 10.0, RED);
    draw_circle(game.player_x * 200.0 + x, game.player_y * 200.0 + y, 4.0, SKYBLUE);
}

fn draw_genome(genome : &Genome, x: f32, y: f32) {

    const SPACING_X: f32 = 100.0;
    const SPACING_Y: f32 = 80.0;

    // convert the node layers to actual positions on the screen
    let mut node_positions: Vec<(f32, f32, NodeIndex)> = Vec::new();

    let mut layers: HashMap<i32, i32> = HashMap::new();
    for node in genome.graph.node_indices() {
        let node_layer = genome.graph.node_weight(node).unwrap().layer;
        let nx: f32 = SPACING_X * node_layer as f32 + x;
        let ny: f32 = SPACING_Y * *layers.entry(node_layer).or_insert(0) as f32 + y;
        *layers.entry(node_layer).or_insert(0) += 1;
        node_positions.push((nx, ny, node));
    }

    // finding layers for centering
    let mut layers: HashMap<i32, i32> = HashMap::new();
    for node in genome.graph.node_weights() {
        *layers.entry(node.layer).or_insert(0) += 1;
    }

    // Center vertically
    for (_, y, node) in node_positions.iter_mut() {
        *y -= (layers.get(&genome.graph.node_weight(*node).unwrap().layer).unwrap() - 1) as f32 * SPACING_Y / 2.0;
    }

    // Center horizontally
    for (x, _, _) in node_positions.iter_mut() {
        *x -= (layers.len() - 1) as f32 * SPACING_X / 2.0;
    }

    // draw connections
    for (nx, ny, node) in node_positions.iter() {
        for edge in genome.graph.edges_directed(*node, Outgoing) {
            let (ox, oy, _) = node_positions.iter().find(|(_, _, n)| *n == edge.target()).unwrap();
            
            draw_line(
                *nx,
                *ny,
                *ox,
                *oy,
                12.0,
                BLACK
            );
            draw_line(
                *nx,
                *ny,
                *ox,
                *oy,
                6.0,
                Color::new(
                    f32::max(0.0, *edge.weight()),
                    f32::max(0.0, *edge.weight()),
                    f32::max(0.0, f32::abs(*edge.weight())),
                    1.0
                )
            );
        }
    }

    // draw the nodes
    for (x, y, node) in node_positions.iter() {
        let weight: f32 = genome.graph.node_weight(*node).unwrap().value;
        let color = Color::new(
            f32::max(0.0, weight),
            f32::max(0.0, weight),
            f32::max(0.0, f32::abs(weight)),
            1.0
        );
        draw_circle(*x, *y, 25.0, color);
        draw_circle_lines(*x, *y, 25.0, -5.0, if genome.output_nodes.contains(&node) {WHITE} else {BLACK});
    }
}

pub fn rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(x + 5.0, y + 5.0, w, h, Color::from_hex(0x282828));
    draw_rectangle(x, y, w, h, color);
    draw_rectangle_lines(x, y, w, h, 10.0, Color::from_hex(0x000000));
}
