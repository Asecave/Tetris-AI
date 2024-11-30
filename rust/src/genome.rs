use petgraph::{algo::toposort, graph::{DiGraph, NodeIndex}, Direction::Incoming};

#[derive(Clone)]
#[allow(dead_code)]
pub enum ActivationFunction {
    Identity,
    Sigmoid,
    Tanh,
    ReLU,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    Hidden,
    Input(u32),
    Output(u32),
}

#[derive(Clone, Copy)]
pub struct Node {
    pub value: f32,
    pub bias: f32,
    pub node_type: NodeType,
    pub layer: i32,
}

pub type GraphIndexType = u32;

#[derive(Clone)]
pub struct Genome {
    pub graph: DiGraph<Node, f32, GraphIndexType>,
    pub topological_order: Vec<NodeIndex>,
    pub input_nodes: Vec<NodeIndex>,
    pub output_nodes: Vec<NodeIndex>,
}

impl Genome {
    pub fn new(graph: DiGraph<Node, f32, GraphIndexType>, input_nodes: Vec<NodeIndex>, output_nodes: Vec<NodeIndex>) -> Self {
        let mut g = Self {
            graph,
            topological_order: Vec::new(),
            input_nodes,
            output_nodes,
        };
        g.sort_topological();
        g
    }

    pub fn sort_topological(&mut self) {
        self.topological_order = toposort(&self.graph, None).unwrap()
    }

    pub fn generate_layers(&mut self) {
        self.sort_topological();
        let mut layer = 1;

        // reset all layers
        for node_index in self.topological_order.iter() {
            // ignore input layer
            if self.input_nodes.contains(node_index) {
                continue;
            }
            if self.output_nodes.contains(node_index) {
                self.graph.node_weight_mut(*node_index).unwrap().layer = i32::MAX;
                continue; 
            }
            self.graph.node_weight_mut(*node_index).unwrap().layer = 1;
        }

        let mut largest_node_layer = 0;

        // assign layers
        for node_index in self.topological_order.iter() {
            // ignore io layers
            if self.input_nodes.contains(node_index) || self.output_nodes.contains(node_index) {
                continue;
            }
            for incoming_node in self.graph.neighbors_directed(*node_index, Incoming) {
                if self.graph.node_weight(incoming_node).unwrap().layer == layer {
                    layer += 1;
                    break;
                }
            }
            self.graph.node_weight_mut(*node_index).unwrap().layer = layer;
            largest_node_layer = i32::max(largest_node_layer, layer);
        }
        for node_index in self.output_nodes.iter() {
            self.graph.node_weight_mut(*node_index).unwrap().layer = largest_node_layer + 1;
        }
    }

    pub fn traverse(&mut self) {

        for index in self.topological_order.iter() {
            
            let mut node = *self.graph.node_weight(*index).unwrap();
            node.value += node.bias;
            node.value = f32::tanh(node.value);
            *self.graph.node_weight_mut(*index).unwrap() = node;

            let mut edges = self.graph.neighbors(*index).detach();
            while let Some((edge, second_node)) = edges.next(&self.graph) {
                let (node_weight, edge_weight) = self.graph.index_twice_mut(second_node, edge);
                node_weight.value += *edge_weight * node.value;
            }
        }
    }
}
