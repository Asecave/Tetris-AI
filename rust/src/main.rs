mod genome;
mod agent;
mod game;
mod ui;

use std::{sync::{Arc, Mutex}, thread, time::{SystemTime, UNIX_EPOCH}};

use game::chase_point::ChasePoint;
use genome::Node;
use agent::Agent;
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub const NUM_AGENTS: usize = 1000;
pub const FRAMES_PER_GEN: u32 = 1000;
pub const MAX_NODES: u32 = 8;
pub const MAX_EDGES: u32 = 8;

#[derive(Clone)]
struct UIShared {
    agent: Option<Agent>,
    generation: u32,
    best_fitness: f32,
    current_frame: u32,
    current_fitness: f32,
    last_evaluation_time: u128,
    last_selection_mutation_time: u128,

    // User Input
    sleep_time: u64,
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
        last_evaluation_time: 0,
        last_selection_mutation_time: 0,
        sleep_time: 10,
    };
    let ui_shared_ref: Arc<Mutex<UIShared>> = Arc::new(Mutex::new(ui_shared));

    let ui_shared_ref_1 = Arc::clone(&ui_shared_ref);
    thread::spawn(move || {
        evolution(&ui_shared_ref_1);
    });
    ui::open_ui(Arc::clone(&Arc::clone(&ui_shared_ref))).await;
}

fn evolution(ui_shared: &Arc<Mutex<UIShared>>) {

    let mut population: Vec<Agent> = Vec::with_capacity(NUM_AGENTS);

    for _ in 0..population.capacity() {
        population.push(Agent::new());
    }

    let mut generation = 0;
    let mut last_evaluation_time = 0;
    let mut last_selection_mutation_time = 0;

    loop {

        {
            let mut ui_shared = ui_shared.lock().unwrap();
            ui_shared.generation = generation;
            ui_shared.best_fitness = population[0].fitness;
            ui_shared.last_evaluation_time = last_evaluation_time;
            ui_shared.last_selection_mutation_time = last_selection_mutation_time;
        }

        let start = SystemTime::now();
        evaluation(&mut population, ui_shared);
        last_evaluation_time = SystemTime::now().duration_since(start).unwrap().as_millis();

        let start = SystemTime::now();
        population = select_and_mutate(&mut population);
        last_selection_mutation_time = SystemTime::now().duration_since(start).unwrap().as_millis();

        for agent in population.iter_mut() {
            agent.game = ChasePoint::new();
        }

        generation += 1;
    }
}

fn evaluation(population: &mut Vec<Agent>, ui_shared: &Arc<Mutex<UIShared>>) {
    let mut last_ui_update = 0;
    let mut sleep_time: u64 = 0;

    // reset fitness
    for agent in population.iter_mut() {
        agent.fitness = 0.0;
    }

    for current_frame in 0..FRAMES_PER_GEN {

        population.par_iter_mut().for_each(|agent| agent.play());

        // Update ui
        if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - last_ui_update >= 1000 / 144 {
            let mut ui_shared = ui_shared.lock().unwrap();
            ui_shared.agent = Some(population[0].clone());
            ui_shared.current_frame = current_frame;
            ui_shared.current_fitness = population[0].fitness;
            last_ui_update = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            sleep_time = ui_shared.sleep_time;
        }

        thread::sleep(std::time::Duration::from_millis(sleep_time));
    }
}

fn select_and_mutate(population: &mut Vec<Agent>) -> Vec<Agent> {

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let move_straight_to_next_gen = (population.len() as f32 * 0.1) as usize;
    let selection_probability = 1.0 / (population.len() as f32 * 0.5);

    let next_generation = population.split_at(move_straight_to_next_gen).0;
    let mut next_generation = next_generation.to_vec();

    let mut i = 0;
    while next_generation.len() < population.len() {
        if thread_rng().gen::<f32>() < selection_probability {
            next_generation.push(mutate_agent(population[i].clone()));
        }
        
        i = (i + 1) % population.len();
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

    if num_nodes < MAX_NODES as usize && num_edges > 0 && thread_rng().gen_bool(0.02) {
        if thread_rng().gen_bool(0.5) {
            // new node
            let random_edge = agent.genome.connections[thread_rng().gen_range(0..num_edges)];
            let old_edge_target = agent.genome.nodes[random_edge.0].connections[random_edge.1].0;
            
            let mut new_node = Node::new();
            new_node.bias = thread_rng().gen_range(-1.0..1.0);
            new_node.connections.push((old_edge_target, thread_rng().gen_range(-1.0..1.0)));

            agent.genome.nodes.push(new_node);

            agent.genome.nodes[random_edge.0].connections[random_edge.1] = (agent.genome.nodes.len() - 1, thread_rng().gen_range(-1.0..1.0));
        } else {
            // remove node
            let random_node = thread_rng().gen_range(0..num_nodes);
            if agent.genome.nodes[random_node].layer > 0 && !agent.genome.output_nodes.contains(&random_node) {
                agent.genome.nodes.remove(random_node);
                for node in agent.genome.nodes.iter_mut() {
                    node.connections.retain(|con| con.0 != random_node);
                    for con in node.connections.iter_mut() {
                        if con.0 > random_node {
                            con.0 -= 1;
                        }
                    }
                }
            }
        }
    }

    agent.genome.build();
    let num_nodes = agent.genome.nodes.len();
    let num_edges = agent.genome.connections.len();

    if num_edges < MAX_EDGES as usize && thread_rng().gen_bool(0.6) {
        if thread_rng().gen_bool(0.5) {
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
            }
        } else if num_edges > 0 {
            // remove connection
            let random_edge = agent.genome.connections[thread_rng().gen_range(0..num_edges)];
            agent.genome.nodes[random_edge.0].connections.remove(random_edge.1);
        }
    }

    agent.genome.build();
    let mut num_nodes = agent.genome.nodes.len();

    // remove dangling nodes
    // let mut found_dangling = false;
    // loop {
    //     for i in 0..num_nodes {
    //         if agent.genome.nodes[i].layer > 0 && !agent.genome.output_nodes.contains(&i) && agent.genome.nodes[i].connections.is_empty() {
    //             found_dangling = true;
    //             agent.genome.nodes.remove(i);
    //             num_nodes -= 1;
    //             for (j, node) in agent.genome.nodes.iter_mut().enumerate() {
    //                 node.connections.retain(|con| con.0 != i);
    //                 for con in node.connections.iter_mut() {
    //                     if con.0 > j {
    //                         con.0 -= 1;
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //     }
    //     if !found_dangling {
    //         break;
    //     }
    //     found_dangling = false;
    // }

    agent.genome.build();

    return agent;
}

// fn test_genome() {
//     let mut nodes: Vec<Node> = Vec::with_capacity(10);

//     for _ in 0..nodes.capacity() {
//         let mut node = Node::new();
//         node.activation_function = ActivationFunction::Tanh;
//         nodes.push(node);
//     }

//     nodes[0].connections.append(&mut vec![(6, 1.0), (8, 1.0)]);
//     nodes[1].connections.append(&mut vec![(8, 1.0)]);
//     nodes[2].connections.append(&mut vec![(9, 1.0)]);
//     nodes[3].connections.append(&mut vec![(8, 1.0), (9, 1.0)]);
//     nodes[4].connections.append(&mut vec![]);
//     nodes[5].connections.append(&mut vec![]);
//     nodes[6].connections.append(&mut vec![(4, 1.0)]);
//     nodes[7].connections.append(&mut vec![(4, 1.0), (5, 1.0)]);
//     nodes[8].connections.append(&mut vec![(6, 1.0), (7, 1.0), (9, 1.0)]);
//     nodes[9].connections.append(&mut vec![(4, 1.0), (5, 1.0)]);

//     let output_nodes: Vec<usize> = vec![4, 5];

//     let mut genome = Genome::new(nodes, output_nodes);

//     genome.set_node_value(0, 1.0);
//     genome.set_node_value(1, 1.0);
//     genome.set_node_value(2, 1.0);
//     genome.set_node_value(3, 1.0);
    
//     genome.traverse();
// }
