mod genome;
mod agent;
mod game;
mod ui;
mod config;

use std::{collections::HashSet, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use agent::Agent;
use game::Game;
use genome::{Node, NodeType};
use macroquad::input::KeyCode;
use rand::{seq::IteratorRandom, thread_rng, Rng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Default)]
struct UIShared {
    population: Option<Vec<Agent>>,
    generation: u32,
    best_fitness: f32,
    current_frame: u32,
    current_fitness: f32,
    last_evaluation_time: u128,
    last_selection_mutation_time: u128,
    mutation_probability: f64,
    sleep_time: u64,
    keyboard_input: HashSet<KeyCode>,
}

#[macroquad::main("AI")]
async fn main() {
    // test_genome();

    let ui_shared = UIShared {
        sleep_time: 10,
        ..Default::default()
    };
    let ui_shared_ref: Arc<Mutex<UIShared>> = Arc::new(Mutex::new(ui_shared));

    let ui_shared_ref_1 = Arc::clone(&ui_shared_ref);
    thread::spawn(move || {
        evolution(&ui_shared_ref_1);
    });
    ui::open_ui(Arc::clone(&Arc::clone(&ui_shared_ref))).await;
}

fn evolution(ui_shared: &Arc<Mutex<UIShared>>) {

    let mut population: Vec<Agent> = Vec::with_capacity(config::NUM_AGENTS);

    for _ in 0..population.capacity() {
        population.push(Agent::new());
    }

    let mut generation = 0;
    let mutation_probability = 1.0;
    let mut last_evaluation_time = 0;
    let mut last_selection_mutation_time = 0;

    loop {

        {
            let mut ui_shared = ui_shared.lock().unwrap();
            ui_shared.generation = generation;
            ui_shared.best_fitness = population[0].fitness;
            ui_shared.last_evaluation_time = last_evaluation_time;
            ui_shared.last_selection_mutation_time = last_selection_mutation_time;
            ui_shared.mutation_probability = mutation_probability;
        }

        let start = SystemTime::now();
        evaluation(&mut population, ui_shared);
        last_evaluation_time = SystemTime::now().duration_since(start).unwrap().as_millis();

        let start = SystemTime::now();
        population = select_and_mutate(&mut population, mutation_probability);
        last_selection_mutation_time = SystemTime::now().duration_since(start).unwrap().as_millis();

        // if mutation_probability > 0.01 {
        //     mutation_probability -= 0.001;
        // }

        for agent in population.iter_mut() {
            agent.game = config::GAME::new();
        }

        generation += 1;
    }
}

fn evaluation(population: &mut Vec<Agent>, ui_shared: &Arc<Mutex<UIShared>>) {
    let mut last_ui_update = 0;
    let mut sleep_time: u64 = ui_shared.lock().unwrap().sleep_time;
    let mut skip_to_next_gen = false;

    // reset fitness
    for agent in population.iter_mut() {
        agent.fitness = 0.0;
    }

    let display_agent = 0;

    for current_frame in 0..config::FRAMES_PER_GEN {

        if config::USE_PARALLELISM {
            population.par_iter_mut().for_each(|agent| agent.play());
        } else {
            population.iter_mut().for_each(|agent| agent.play());
        }

        // Update ui
        if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - last_ui_update >= 1000 / 144 {
            let mut ui_shared = ui_shared.lock().unwrap();

            if ui_shared.keyboard_input.get(&KeyCode::Up).is_some() {
                sleep_time += 10;
            }
            if ui_shared.keyboard_input.get(&KeyCode::Down).is_some() && sleep_time > 0 {
                sleep_time -= 10;
            }
            if ui_shared.keyboard_input.get(&KeyCode::Right).is_some() {
                skip_to_next_gen = true;
                sleep_time = 0;
            }
            ui_shared.keyboard_input.clear();

            ui_shared.population = Some(population.clone());
            ui_shared.current_frame = current_frame;
            ui_shared.current_fitness = population[display_agent].fitness;
            ui_shared.sleep_time = sleep_time;

            last_ui_update = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        }

        thread::sleep(Duration::from_millis(sleep_time));
    }
    if skip_to_next_gen {
        ui_shared.lock().unwrap().sleep_time = 10;
    }
}

fn select_and_mutate(population: &mut Vec<Agent>, mutation_probability: f64) -> Vec<Agent> {

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let move_straight_to_next_gen = (population.len() as f32 * 0.3) as usize;
    let selection_probability = 1.0 / (population.len() as f32 * 0.5);

    let next_generation = population.split_at(move_straight_to_next_gen).0;
    let mut next_generation = next_generation.to_vec();

    let mut i = 0;
    while next_generation.len() < population.len() {
        if thread_rng().gen::<f32>() < selection_probability {
            next_generation.push(mutate_agent(population[i].clone_and_keep_io_nodes(), mutation_probability));
            i = 0;
            continue;
        }
        
        i = (i + 1) % population.len();
    }

    return next_generation;
}

fn mutate_agent(mut agent: Agent, mutation_probability: f64) -> Agent {

    // mutate bias
    if thread_rng().gen_bool(0.15 * mutation_probability) {
        let random_node = agent.genome.graph.node_indices().choose(&mut thread_rng()).unwrap();
        if thread_rng().gen_bool(0.25 * mutation_probability) {
            // big change
            agent.genome.graph.node_weight_mut(random_node).unwrap().bias += thread_rng().gen_range(-1.0..1.0);
        } else {
            // small change
            agent.genome.graph.node_weight_mut(random_node).unwrap().bias += thread_rng().gen_range(-0.2..0.2);
        }
    }

    // mutate weight
    if agent.genome.graph.edge_count() > 0 && thread_rng().gen_bool(0.15 * mutation_probability) {
        let random_edge = agent.genome.graph.edge_indices().choose(&mut thread_rng()).unwrap();
        if thread_rng().gen_bool(0.25 * mutation_probability) {
            // big change
            *agent.genome.graph.edge_weight_mut(random_edge).unwrap() += thread_rng().gen_range(-1.0..1.0);
        } else {
            // small change
            *agent.genome.graph.edge_weight_mut(random_edge).unwrap() += thread_rng().gen_range(-0.2..0.2);
        }
    }
    
    // remove node
    if thread_rng().gen_bool(0.01 * mutation_probability) {
        let random_node = agent.genome.graph.node_indices().choose(&mut thread_rng()).unwrap();
        if !agent.genome.input_nodes.contains(&random_node) && !agent.genome.output_nodes.contains(&random_node) {
            agent.genome.graph.remove_node(random_node);
        }
    }

    // new node
    if agent.genome.graph.node_count() < config::MAX_NODES as usize && agent.genome.graph.edge_count() > 0 && thread_rng().gen_bool(0.01 * mutation_probability) {

        let random_edge = agent.genome.graph.edge_indices().choose(&mut thread_rng()).unwrap();
        let (left_end, right_end) = agent.genome.graph.edge_endpoints(random_edge).unwrap();
        let removed_edge_weight = *agent.genome.graph.edge_weight(random_edge).unwrap();
        agent.genome.graph.remove_edge(random_edge);
        let new_node = agent.genome.graph.add_node(Node {
            value: 0.0,
            bias: 1.0,
            node_type: NodeType::Hidden,
            layer: 1
        });
        agent.genome.graph.add_edge(left_end, new_node, removed_edge_weight);
        agent.genome.graph.add_edge(new_node, right_end, 1.0);
    }
    
    agent.genome.generate_layers();

    // remove connection
    if agent.genome.graph.edge_count() > 0 && thread_rng().gen_bool(0.1 * mutation_probability) {
        let random_edge = agent.genome.graph.edge_indices().choose(&mut thread_rng()).unwrap();
        agent.genome.graph.remove_edge(random_edge);
    }

    // new conenction
    if agent.genome.graph.edge_count() < config::MAX_EDGES as usize && thread_rng().gen_bool(0.1 * mutation_probability) {

        let mut first_node;
        loop {
            first_node = agent.genome.graph.node_indices().choose(&mut thread_rng()).unwrap();
            if !agent.genome.output_nodes.contains(&first_node) {
                break;
            }
        }
        let first_node_layer = agent.genome.graph.node_weight(first_node).unwrap().layer;
        let mut second_node;
        loop {
            second_node = agent.genome.graph.node_indices().choose(&mut thread_rng()).unwrap();
            let layer = agent.genome.graph.node_weight(second_node).unwrap().layer;
            if !agent.genome.input_nodes.contains(&second_node) && layer > first_node_layer {
                break;
            }
        }
        agent.genome.graph.update_edge(first_node, second_node, thread_rng().gen_range(-1.0..1.0));
    }

    agent.genome.generate_layers();

    return agent;
}

// fn test_genome() {

//     let mut graph = DiGraph::new();

//     let n0 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Input(0), layer: 0});
//     let n1 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Input(1), layer: 0});
//     let n2 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Input(2), layer: 0});
//     let n3 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Input(3), layer: 0});
//     let n4 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Output(0), layer: 1});
//     let n5 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Output(1), layer: 1});
//     let n6 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Hidden, layer: 1});
//     let n7 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Hidden, layer: 1});
//     let n8 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Hidden, layer: 1});
//     let n9 = graph.add_node(Node {value: 0.0, bias: 0.0, node_type: NodeType::Hidden, layer: 1});

//     graph.add_edge(n0, n6, 0.0);
//     graph.add_edge(n0, n8, 0.0);
//     graph.add_edge(n1, n8, 0.0);
//     graph.add_edge(n2, n9, 0.0);
//     graph.add_edge(n3, n8, 0.0);
//     graph.add_edge(n3, n9, 0.0);
//     graph.add_edge(n8, n6, 0.0);
//     graph.add_edge(n8, n7, 0.0);
//     graph.add_edge(n8, n9, 0.0);
//     graph.add_edge(n6, n4, 0.0);
//     graph.add_edge(n7, n4, 0.0);
//     graph.add_edge(n7, n5, 0.0);
//     graph.add_edge(n9, n4, 0.0);
//     graph.add_edge(n9, n5, 0.0);

//     let input_nodes = vec![n0, n1, n2, n3];
//     let output_nodes = vec![n4, n5];

//     let mut g = Genome::new(graph, input_nodes, output_nodes);

//     g.generate_layers();
// }
