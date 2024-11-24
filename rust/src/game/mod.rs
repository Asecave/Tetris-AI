use crate::genome::Genome;

pub mod chase_point;
pub trait Game {
    fn create_base_genome(&self) -> Genome;
    fn set_input_node_values(&self, genome: &mut Genome);
    fn update(&mut self);
}