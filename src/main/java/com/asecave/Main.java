package com.asecave;

class Main {
	
	public static void main(String[] args) {
		
		Tetris tetris = new Tetris();
		
		for (int i = 0; i < 10; i++) {
			tetris.gravity();
			
			byte[] board = tetris.getBoard();
			
			for (int j = 0; j < board.length; j++) {
				System.out.print(String.format("%8s", Integer.toBinaryString(board[j] & 0xFF)).replace(' ', '0'));
				if (j % 3 == 2) System.out.println();
			}
			System.out.println();
			
			try {
				Thread.sleep(100);
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}
    }
	
}