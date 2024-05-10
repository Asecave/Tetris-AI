package com.asecave;

public class Tetris {

	private byte[] board;
	
	private byte currentPiece;
	private byte rotation;
	
	private byte[] bag, nextBag;
	
	private byte posX, posY;
	
	private static final byte[][][] TETROMINOS = {
		{
			{0, 0, 0, 0}, // I
			{0, 0, 0, 0},
			{1, 1, 1, 1},
			{0, 0, 0, 0},
		},{
			{0, 1, 0, 0}, // I
			{0, 1, 0, 0},
			{0, 1, 0, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // I
			{1, 1, 1, 1},
			{0, 0, 0, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 1, 0}, // I
			{0, 0, 1, 0},
			{0, 0, 1, 0},
			{0, 0, 1, 0},
		},{
			{0, 0, 0, 0}, // J
			{1, 0, 0, 0},
			{1, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // J
			{0, 1, 1, 0},
			{0, 1, 0, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // J
			{0, 0, 0, 0},
			{1, 1, 1, 0},
			{0, 0, 1, 0},
		},{
			{0, 0, 0, 0}, // J
			{0, 1, 0, 0},
			{0, 1, 0, 0},
			{1, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // L
			{0, 0, 1, 0},
			{1, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // L
			{0, 1, 0, 0},
			{0, 1, 0, 0},
			{0, 1, 1, 0},
		},{
			{0, 0, 0, 0}, // L
			{0, 0, 0, 0},
			{1, 1, 1, 0},
			{1, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // L
			{1, 1, 0, 0},
			{0, 1, 0, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // O
			{0, 1, 1, 0},
			{0, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // O
			{0, 1, 1, 0},
			{0, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // O
			{0, 1, 1, 0},
			{0, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // O
			{0, 1, 1, 0},
			{0, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // S
			{0, 1, 1, 0},
			{1, 1, 0, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // S
			{0, 1, 0, 0},
			{0, 1, 1, 0},
			{0, 0, 1, 0},
		},{
			{0, 0, 0, 0}, // S
			{0, 0, 0, 0},
			{0, 1, 1, 0},
			{1, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // S
			{1, 0, 0, 0},
			{1, 1, 0, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // T
			{0, 1, 0, 0},
			{1, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // T
			{0, 1, 0, 0},
			{0, 1, 1, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // T
			{0, 0, 0, 0},
			{1, 1, 1, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // T
			{0, 1, 0, 0},
			{1, 1, 0, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // Z
			{1, 1, 0, 0},
			{0, 1, 1, 0},
			{0, 0, 0, 0},
		},{
			{0, 0, 0, 0}, // Z
			{0, 0, 1, 0},
			{0, 1, 1, 0},
			{0, 1, 0, 0},
		},{
			{0, 0, 0, 0}, // Z
			{0, 0, 0, 0},
			{1, 1, 0, 0},
			{0, 1, 1, 0},
		},{
			{0, 0, 0, 0}, // Z
			{0, 1, 0, 0},
			{1, 1, 0, 0},
			{1, 0, 0, 0},
		},
	};
	
	public Tetris() {
		
		board = new byte[3 * 10];
		bag = generateBag();
		nextBag = generateBag();
	}
	
	public void gravity() {
		
		
	}
	
	private void placePiece() {
		
		byte piece = (byte) (bag[currentPiece] * 4 + rotation);
		
		if (currentPiece == 8) {
			bag = nextBag;
			nextBag = generateBag();
			currentPiece = 0;
		}
	}
	
	public void hardDrop() {
		// TODO dropping
	}
	
	private byte[] generateBag() {
		byte[] newBag = new byte[7];
		for (byte i = 0; i < newBag.length; i++) {
			newBag[i] = i;
		}
		for (byte i = 0; i < newBag.length; i++) {
			byte random = (byte) (Math.random() * newBag.length);
			byte tmp = newBag[i];
			newBag[i] = newBag[random];
			newBag[random] = tmp;
		}
		return newBag;
	}
	
	public byte[] getBoard() {
		return board.clone();
	}
}
