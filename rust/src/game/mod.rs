use crate::genome::Genome;

pub mod chase_point;
pub mod maze;
pub trait Game {
    fn base_genome_io() -> (u32, u32);
    fn set_input_node_values(&self, genome: &mut Genome);
    fn apply_outputs(&mut self, genome: &Genome);
    fn update(&mut self);
    fn fitness_function(&mut self, fitness: f32) -> f32;
    fn draw_static(&self, x: f32, y: f32, scale: f32);
    fn draw_dynamic(&self, x: f32, y: f32, scale: f32);
    fn draw_best(&self, x: f32, y: f32, scale: f32);
    fn new() -> Self;
}