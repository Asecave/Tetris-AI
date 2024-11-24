#[derive(Clone)]
#[allow(dead_code)]
pub enum ActivationFunction {
    Identity,
    Sigmoid,
    Tanh,
    ReLU,
}

#[derive(Clone)]
pub struct Node {
    pub bias: f32,
    pub connections: Vec<(usize, f32)>,
    pub activation_function: ActivationFunction,
    pub value: f32,
    pub layer: u32,
}

#[derive(Clone)]
pub struct Genome {
    pub nodes: Vec<Node>,
    pub connections: Vec<(usize, usize)>,
    pub output_nodes: Vec<usize>,
    topological_arrangement: Vec<usize>,
    pub layer_count: u32,
}

impl Node {
    pub fn new() -> Self {
        Self {
            bias: 0.0,
            connections: Vec::new(),
            activation_function: ActivationFunction::Identity,
            value: 0.0,
            layer: 0,
        }
    }

    fn apply_value(&mut self) {
        self.value += self.bias;
        self.value = match self.activation_function {
            ActivationFunction::Identity => {
                self.value
            },
            ActivationFunction::Sigmoid => {
                1.0 / (1.0 + f32::exp(-self.value))
            },
            ActivationFunction::Tanh => {
                f32::tanh(self.value)
            },
            ActivationFunction::ReLU => {
                f32::max(0.0, self.value)
            },
        };
    }
}

impl Genome {
    pub fn new(nodes: Vec<Node>, output_nodes: Vec<usize>) -> Self {
        let mut g = Self {
            nodes,
            connections: Vec::new(),
            output_nodes,
            topological_arrangement: Vec::new(),
            layer_count: 0,
        };
        g.build();
        g
    }

    pub fn build(&mut self) {
        self.sort_topological();
        self.set_layer_count();
        self.set_connections();
    }

    fn set_connections(&mut self) {
        self.connections.clear();
        for (i, node) in self.nodes.iter().enumerate() {
            for j in 0..node.connections.len() {
                self.connections.push((i, j));
            }
        }
    }

    fn set_layer_count(&mut self) {
        let mut layers: Vec<u32> = Vec::new();
        for node in &self.nodes {
            if !layers.contains(&node.layer) {
                layers.push(node.layer);
            }
        }
        self.layer_count = layers.len() as u32;
    }

    fn sort_topological(&mut self) {
        self.topological_arrangement.clear();
        let mut tmp_nodes = self.nodes.clone();
        let len = tmp_nodes.len();
        let mut layer_number = 0;
        while self.topological_arrangement.len() < len {
            let mut layer = Vec::new();
            'outer:
            for i in 0..len {
                if tmp_nodes[i].connections.contains(&(usize::MAX, 0.0)) {
                    continue;
                }
                for (j, node) in tmp_nodes.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    if node.connections.iter().any(|(connection, _)| *connection as usize == i) {
                        continue 'outer;
                    }
                }
                layer.push(i);
            }
            for i in &layer {
                tmp_nodes[*i].connections.clear();
                tmp_nodes[*i].connections.push((usize::MAX, 0.0));
                self.nodes[*i].layer = layer_number;
            }
            self.topological_arrangement.append(&mut layer);
            layer_number += 1;
        }
        
        if layer_number == 1 {
            self.nodes[self.output_nodes[0]].layer = 1;
        }
        let largest_output_layer = self.output_nodes.iter().fold(0, |prev, node| u32::max(prev, self.nodes[*node].layer));
        for output_node in self.output_nodes.iter() {
            self.nodes[*output_node].layer = largest_output_layer;
        }
    }

    pub fn traverse(&mut self) -> Vec<f32> {

        let first_non_input_node = self.nodes.iter().position(|node| node.layer > 0).unwrap_or(0);

        for i in first_non_input_node..self.nodes.len() {
            self.nodes[i].value = 0.0;
        }
    
        for i in &self.topological_arrangement {
            self.nodes[*i].apply_value();
            for (connected, weight) in self.nodes[*i].connections.clone() {
                self.nodes[connected as usize].value += self.nodes[*i].value * weight;
            }
        }
    
        return self.output_nodes.iter().map(|i| self.nodes[*i].value).collect();
    }

    pub fn set_node_value(&mut self, index: usize, value: f32) {
        self.nodes[index].value = value;
    }
}
