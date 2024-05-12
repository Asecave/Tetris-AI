package com.asecave;

class Main {

	public static void main(String[] args) {

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

}