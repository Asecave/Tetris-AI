package com.asecave;

class Main {

	public static void main(String[] args) {

		SynchronizedTetris tetris = new SynchronizedTetris();
		
		TetrisWindow window = new TetrisWindow();
		
		window.setTetrisGame(tetris);
		
		int frames = 0;
		long lastPrint = System.currentTimeMillis();
		while (true) {
			tetris.gravity();
			frames++;
			if (System.currentTimeMillis() - lastPrint > 1000) {
				System.out.println(frames);
				lastPrint = System.currentTimeMillis();
				frames = 0;
			}
//			frame.repaint();
//			try {
//				Thread.sleep(0, 1);
//			} catch (InterruptedException e) {
//				e.printStackTrace();
//			}
		}
	}

}