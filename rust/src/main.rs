mod genome;
mod agent;
mod game;
mod ui;

use std::{sync::{Arc, Mutex}, thread, time::{SystemTime, UNIX_EPOCH}};

use game::chase_point::ChasePoint;
use genome::{Node, Genome, ActivationFunction};
use agent::Agent;
use rand::{thread_rng, Rng};

pub const NUM_AGENTS: u32 = 100;
pub const FRAMES_PER_GEN: u32 = 200;

#[derive(Clone)]
struct UIShared {
    agent: Option<Agent>,
    generation: u32,
    best_fitness: f32,
    current_frame: u32,
    current_fitness: f32,
}

#[macroquad::main("AI")]
async fn main() {
    //test_genome();

    let ui_shared = UIShared {
        agent: None,
        generation: 0,
        best_fitness: 0.0,
        current_frame: 0,
        current_fitness: 0.0,
    };
    let ui_shared_ref: Arc<Mutex<UIShared>> = Arc::new(Mutex::new(ui_shared));

    let ui_shared_ref_1 = Arc::clone(&ui_shared_ref);
    thread::spawn(move || {
        generation(&ui_shared_ref_1);
    });
    ui::open_ui(Arc::clone(&Arc::clone(&ui_shared_ref))).await;
}

fn generation(ui_shared: &Arc<Mutex<UIShared>>) {

    let mut population: Vec<Agent> = Vec::with_capacity(500);

    for _ in 0..population.capacity() {
        population.push(Agent::new());
    }

    let mut calc_durations= [0u32; NUM_AGENTS as usize];
    let mut current_duration: usize = 0;
    let mut reached_end = false;
    let mut generation = 0;

    loop {

        {
            let mut ui_shared = ui_shared.lock().unwrap();
            ui_shared.generation = generation;
            ui_shared.best_fitness = population[0].fitness;
        }

        let start = SystemTime::now();

        evaluation(&mut population, ui_shared);

        calc_durations[current_duration] = SystemTime::now().duration_since(start).unwrap().as_millis() as u32;
        current_duration = (current_duration + 1) % calc_durations.len();
        if current_duration == 0 {
            reached_end = true;
        }
        if reached_end {
            let average_time = calc_durations.iter().sum::<u32>() as f32 / calc_durations.len() as f32;
            dbg!(average_time);
        }

        population = select_and_mutate(&mut population);

        for agent in population.iter_mut() {
            agent.game = ChasePoint::new();
        }

        generation += 1;
    }
}

fn evaluation(population: &mut Vec<Agent>, ui_shared: &Arc<Mutex<UIShared>>) {
    let mut last_ui_update = 0;

    // reset fitness
    for agent in population.iter_mut() {
        agent.fitness = 0.0;
    }

    for current_frame in 0..FRAMES_PER_GEN {

        for agent in population.iter_mut() {
            agent.play();
        }

        // Update ui
        if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - last_ui_update >= 1000 / 144 {
            let mut ui_shared = ui_shared.lock().unwrap();
            ui_shared.agent = Some(population[0].clone());
            ui_shared.current_frame = current_frame;
            ui_shared.current_fitness = population[0].fitness;
            last_ui_update = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        }

        //thread::sleep(std::time::Duration::from_millis(f32::floor(1000.0 / 144.0) as u64));
    }
}

fn select_and_mutate(population: &mut Vec<Agent>) -> Vec<Agent> {

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let move_straight_to_next_gen = (population.len() as f32 * 0.3) as usize;
    let selection_probability = 1.0 / (population.len() as f32 * 0.6);

    let (next_generation, remaining) = population.split_at(move_straight_to_next_gen);
    let mut next_generation = next_generation.to_vec();

    let mut i = 0;
    while next_generation.len() < population.len() {
        if thread_rng().gen::<f32>() < selection_probability {
            next_generation.push(mutate_agent(remaining[i].clone()));
        }
        
        i = (i + 1) % remaining.len();
    }

    return next_generation;
}

fn mutate_agent(mut agent: Agent) -> Agent {
    
    let num_nodes = agent.genome.nodes.len();
    let num_edges = agent.genome.connections.len();

    if thread_rng().gen_bool(0.25) {
        if thread_rng().gen_bool(0.5) {
            // mutate bias
            let random_node = &mut agent.genome.nodes[thread_rng().gen_range(0..num_nodes)];
            if thread_rng().gen_bool(0.25) {
                // big change
                (*random_node).bias += thread_rng().gen_range(-1.0..1.0);
            } else {
                // small change
                (*random_node).bias += thread_rng().gen_range(-0.2..0.2);
            }
        } else if num_edges > 0 {
            // mutate weight
            let random_edge = agent.genome.connections[thread_rng().gen_range(0..num_edges)];
            if thread_rng().gen_bool(0.25) {
                // big change
                agent.genome.nodes[random_edge.0].connections[random_edge.1].1 += thread_rng().gen_range(-1.0..1.0);
            } else {
                // small change
                agent.genome.nodes[random_edge.0].connections[random_edge.1].1 += thread_rng().gen_range(-0.2..0.2);
            }
        }
    }

    let mut rebuild_needed = false;

    if num_edges > 0 && thread_rng().gen_bool(0.05) {
        // new node
        let random_edge = agent.genome.connections[thread_rng().gen_range(0..num_edges)];
        let old_edge_target = agent.genome.nodes[random_edge.0].connections[random_edge.1].0;
        
        let mut new_node = Node::new();
        new_node.bias = thread_rng().gen_range(-1.0..1.0);
        new_node.connections.push((old_edge_target, thread_rng().gen_range(-1.0..1.0)));

        agent.genome.nodes.push(new_node);

        agent.genome.nodes[random_edge.0].connections[random_edge.1] = (agent.genome.nodes.len() - 1, thread_rng().gen_range(-1.0..1.0));

        rebuild_needed = true;
    }

    if thread_rng().gen_bool(0.8) {
        // new conenction

        let mut other_node;

        loop {
            other_node = thread_rng().gen_range(0..num_nodes);

            if agent.genome.nodes[other_node].layer > 0 {
                break;
            }
        }

        let other_node_layer = agent.genome.nodes[other_node].layer;

        let mut first_node;
        loop {
            first_node = &mut agent.genome.nodes[thread_rng().gen_range(0..num_nodes)];

            if first_node.layer < other_node_layer {
                break;
            }
        }

        if !first_node.connections.iter().any(|con| con.0 == other_node) {
            first_node.connections.push((other_node, thread_rng().gen_range(-1.0..1.0)));

            rebuild_needed = true;
        }
    }

    if rebuild_needed {
        agent.genome.build();
    }

    return agent;
}

fn test_genome() {
    let mut nodes: Vec<Node> = Vec::with_capacity(10);

    for _ in 0..nodes.capacity() {
        let mut node = Node::new();
        node.activation_function = ActivationFunction::Tanh;
        nodes.push(node);
    }

    nodes[0].connections.append(&mut vec![(6, 1.0), (8, 1.0)]);
    nodes[1].connections.append(&mut vec![(8, 1.0)]);
    nodes[2].connections.append(&mut vec![(9, 1.0)]);
    nodes[3].connections.append(&mut vec![(8, 1.0), (9, 1.0)]);
    nodes[4].connections.append(&mut vec![]);
    nodes[5].connections.append(&mut vec![]);
    nodes[6].connections.append(&mut vec![(4, 1.0)]);
    nodes[7].connections.append(&mut vec![(4, 1.0), (5, 1.0)]);
    nodes[8].connections.append(&mut vec![(6, 1.0), (7, 1.0), (9, 1.0)]);
    nodes[9].connections.append(&mut vec![(4, 1.0), (5, 1.0)]);

    let output_nodes: Vec<usize> = vec![4, 5];

    let mut genome = Genome::new(nodes, output_nodes);

    genome.set_node_value(0, 1.0);
    genome.set_node_value(1, 1.0);
    genome.set_node_value(2, 1.0);
    genome.set_node_value(3, 1.0);
    
    genome.traverse();
}
