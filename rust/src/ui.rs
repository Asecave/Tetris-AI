use std::sync::{Arc, Mutex};

use macroquad::prelude::*;
use miniquad::window::set_window_size;

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

    let mut node_positions: Vec<(f32, f32)> = Vec::new();
    node_positions.resize(genome.nodes.len(), Default::default());

    let mut node_counts_per_layer: Vec<u32> = Vec::new();
    node_counts_per_layer.resize(genome.layer_count as usize, Default::default());

    for (i, node) in genome.nodes.iter().enumerate() {

        node_positions[i].0 = node.layer as f32 * SPACING_X + x;

        let layer_node_count = &mut node_counts_per_layer[node.layer as usize];
        node_positions[i].1 = *layer_node_count as f32 * SPACING_Y + y;
        *layer_node_count += 1;
    }

    // Make columns of Nodes vertically centered
    let largest_column = node_counts_per_layer.iter().copied().fold(0, u32::max);
    for (i, node) in genome.nodes.iter().enumerate() {
        node_positions[i].1 -= ((largest_column - 1) as f32 * SPACING_Y) / 2.0;
        if node_counts_per_layer[node.layer as usize] == largest_column {
            continue;
        }
        node_positions[i].1 += (largest_column as f32 * SPACING_Y) / 2.0 - (node_counts_per_layer[node.layer as usize] as f32 * SPACING_Y) / 2.0;
    }

    // Center horizontally
    let offset = ((genome.layer_count - 1) as f32 * SPACING_X) / 2.0;
    for i in 0..node_positions.len() {
        node_positions[i].0 -= offset;
    }

    // Actual drawing
    for draw_circles in 0..=1 {
        for (i, node) in genome.nodes.iter().enumerate() {
            if draw_circles == 1 {
                draw_circle(
                    node_positions[i].0,
                    node_positions[i].1,
                    25.0,
                    Color::new(
                        0.0,
                        f32::max(0.0, node.value),
                        f32::max(0.0, -node.value),
                        1.0
                    )
                );
                draw_circle_lines(
                    node_positions[i].0,
                    node_positions[i].1,
                    25.0,
                    -5.0,
                    BLACK
                );
            } else {
                for (connection, weight) in &node.connections {
                    draw_line(
                        node_positions[i].0,
                        node_positions[i].1,
                        node_positions[*connection as usize].0,
                        node_positions[*connection as usize].1,
                        12.0,
                        BLACK
                    );
                    draw_line(
                        node_positions[i].0,
                        node_positions[i].1,
                        node_positions[*connection as usize].0,
                        node_positions[*connection as usize].1,
                        6.0,
                        Color::new(
                            0.0,
                            f32::max(0.0, *weight),
                            f32::max(0.0, -*weight),
                            1.0
                        )
                    );
                }
            }
        }
    }
}

pub fn rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(x + 5.0, y + 5.0, w, h, Color::from_hex(0x282828));
    draw_rectangle(x, y, w, h, color);
    draw_rectangle_lines(x, y, w, h, 10.0, Color::from_hex(0x000000));
}
