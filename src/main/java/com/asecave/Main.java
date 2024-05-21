package com.asecave;

import java.util.List;

import com.vadeen.neat.Neat;
import com.vadeen.neat.genome.Genome;
import com.vadeen.neat.genome.GenomeEvaluator;
import com.vadeen.neat.species.Species;

class Main {

	public Main() {

		GenomeEvaluator evaluator = new GenomeEvaluator() {
			
			@Override
			public void evaluateAll(List<Species> species) {
				for (Species s : species) {
					for (Genome g : s.getGenomes()) {
						
					}
				}
			}
		};
		
		Neat neat = Neat.create(evaluator, 10 * 20 + 7 + 5 * 7, 6);
				
		SynchronizedTetris tetris = new SynchronizedTetris();

		TetrisWindow window = new TetrisWindow();

		window.setTetrisGame(tetris);

		while (!tetris.isGameOver()) {
			tetris.gravity();
			try {
				Thread.sleep(100);
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}
		window.close();
	}

	public static void main(String[] args) {
		new Main();
	}

}