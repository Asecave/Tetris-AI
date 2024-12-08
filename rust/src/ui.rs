use std::{collections::HashMap, sync::{Arc, Mutex}};

use macroquad::prelude::*;
use miniquad::window::set_window_size;
use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction::Outgoing};

use crate::{agent::Agent, config, game::Game, genome::Genome, UIShared};

pub async fn open_ui(ui_shared: Arc<Mutex<UIShared>>) {

    set_window_size(1600, 1000);

    let mut previous_ui: Option<UIShared> = None;
    let mut game_scale;
    let mut large_game = false;
    let mut only_draw_best = false;
    let mut fullscreen = false;

    loop {

        next_frame().await;

        // Input
        {
            let mut ui = ui_shared.lock().unwrap();
            if ui.keyboard_input.len() == 0 {
                ui.keyboard_input = get_keys_pressed();
            }
            if is_key_pressed(KeyCode::F) {
                large_game = !large_game;
            }
            if is_key_pressed(KeyCode::B) {
                only_draw_best = !only_draw_best;
            }
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::F11) {
                fullscreen = !fullscreen;
                set_fullscreen(fullscreen);
            }
        }

        
        let margin = 50.0;
        let game_x;
        let game_y;
        
        if large_game {
            clear_background(Color::from_hex(0x222222));
            game_scale = screen_height() * 0.5;
            game_x = screen_width() * 0.5;
            game_y = screen_height() * 0.5;
        } else {
            clear_background(DARKGRAY);
            game_scale = screen_height() * 0.25 - margin;
            game_x = screen_width() * 0.75;
            game_y = screen_height() * 0.25 + margin * 0.25;
            rect(margin, margin, screen_width() / 2.0 - margin * 1.5, screen_height() / 2.0 - margin * 1.5, LIGHTGRAY);
            rect(margin, screen_height() / 2.0 + margin * 0.5, screen_width() / 2.0 - margin * 1.5, screen_height() / 2.0 - margin * 1.5, LIGHTGRAY);
            rect(screen_width() / 2.0 + margin * 0.5, margin, screen_width() / 2.0 - margin * 1.5, screen_height() / 2.0 - margin * 1.5, LIGHTGRAY);
        }
        
        if let Ok(ui) = ui_shared.try_lock() {
            previous_ui = Some((*ui).clone());
        }
        let Some(ui) = previous_ui.clone() else {
            continue;
        };
        let Some(population) = &ui.population else {
            continue;
        };
        draw_game(&population, game_x, game_y, game_scale, only_draw_best);
        if large_game {
            continue;
        }

        draw_info_text(&ui, margin + 10.0, margin + 10.0);
        draw_genome(&population[0].genome, screen_width() * 0.25, screen_height() * 0.75 - margin * 0.25);
    }
}

fn draw_game(population: &Vec<Agent>, x: f32, y: f32, game_scale: f32, only_draw_best: bool) {
    population[0].game.draw_static(x, y, game_scale);
    if !only_draw_best {
        for agent in population.iter() {
            agent.game.draw_dynamic(x, y, game_scale);
        }
    }
    population[0].game.draw_best(x, y, game_scale);
}

fn draw_info_text(ui: &UIShared, x: f32, y: f32) {
    let mut info_text = String::new();

    info_text.push_str(format!("Generation: {}\n", &ui.generation).as_str());
    info_text.push_str(format!("Best fitness: {:.1}\n", &ui.best_fitness).as_str());
    info_text.push_str(format!("Frame: {}/{}\n", &ui.current_frame, config::FRAMES_PER_GEN).as_str());
    info_text.push_str(format!("Current fitness: {:.1}\n", &ui.current_fitness).as_str());
    info_text.push_str(format!("Last evaluation time: {}ms\n", &ui.last_evaluation_time).as_str());
    info_text.push_str(format!("Last selection & mutation time: {}ms\n", &ui.last_selection_mutation_time).as_str());
    info_text.push_str(format!("Sleep time: {}ms\n", &ui.sleep_time).as_str());
    info_text.push_str(format!("Mutation Probability: {:.1}%\n", &ui.mutation_probability * 100.0).as_str());

    let mut text_row_offset = 0.0;
    for line in info_text.split("\n") {
        draw_text(line, x, text_row_offset + y + 30.0, 30.0, BLACK);
        text_row_offset += 30.0;
    }
}

fn draw_genome(genome : &Genome, x: f32, y: f32) {

    const SPACING_X: f32 = 75.0;
    const SPACING_Y: f32 = 75.0;

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
        draw_circle(*x, *y, 20.0, color);
        draw_circle_lines(*x, *y, 20.0, -5.0, if genome.output_nodes.contains(&node) {WHITE} else {BLACK});
    }
}

pub fn rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(x + 5.0, y + 5.0, w, h, Color::from_hex(0x282828));
    draw_rectangle(x, y, w, h, color);
    draw_rectangle_lines(x, y, w, h, 10.0, Color::from_hex(0x000000));
}
