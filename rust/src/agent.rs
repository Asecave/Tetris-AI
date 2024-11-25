use crate::{game::{chase_point::ChasePoint, Game}, genome::Genome};

#[derive(Clone)]
pub struct Agent {
    pub genome: Genome,
    pub game: ChasePoint,
    pub fitness: f32,
}

impl Agent {

    pub fn new() -> Self {
        let game = ChasePoint::new();
        Self {
            genome: game.create_base_genome(),
            game,
            fitness: 0.0,
        }
    }

    pub fn play(&mut self) {

        self.game.set_input_node_values(&mut self.genome);
        let output = self.genome.traverse();
        self.game.move_x(output[0]);
        self.game.move_y(output[1]);
        self.game.update();

        self.fitness = self.fitness_function();
    }

    fn fitness_function(&self) -> f32 {
        let mut fitness = 1.0 / (self.game.target_distance2() + 1.0);
        fitness *= f32::powf(0.5, f32::abs(self.game.vel_x) + f32::abs(self.game.vel_y)); 
        if self.game.total_distance > 2.0 {
            fitness /= self.game.total_distance;
        }
        fitness
    }
}