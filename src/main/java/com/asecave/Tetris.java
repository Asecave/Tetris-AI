package com.asecave;

public class Tetris {

	private int[] board;

	private int currentPiece;
	private int rotation;

	private int[] bag, nextBag;

	private int posX, posY;
	private int currentLockDownPass;
	private int hold;
	private int lockDownResets;
	private int totalLinesSent;
	private int lowestY;
	
	private static final int START_X = 3;
	private static final int START_Y = -4;
	private static final int LOCK_DOWN_PASSES = 4;
	private static final int MAX_LOCK_DOWN_RESETS = 15;

	// @formatter:off
	private static final int[][][] TETROMINOS = { 
			{ 
				{ 0, 0, 0, 0 }, // I
				{ 1, 1, 1, 1 }, 
				{ 0, 0, 0, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 1, 0 }, // I
				{ 0, 0, 1, 0 }, 
				{ 0, 0, 1, 0 }, 
				{ 0, 0, 1, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // I
				{ 0, 0, 0, 0 }, 
				{ 1, 1, 1, 1 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 1, 0, 0 }, // I
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // J
				{ 1, 0, 0, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // J
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // J
				{ 0, 0, 0, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 0, 0, 1, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // J
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
				{ 1, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // L
				{ 0, 0, 1, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // L
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // L
				{ 0, 0, 0, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 1, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // L
				{ 1, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // O
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // O
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // O
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // O
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // S
				{ 0, 1, 1, 0 }, 
				{ 1, 1, 0, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // S
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 1, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // S
				{ 0, 0, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 1, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // S
				{ 1, 0, 0, 0 }, 
				{ 1, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // T
				{ 0, 1, 0, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // T
				{ 0, 1, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // T
				{ 0, 0, 0, 0 }, 
				{ 1, 1, 1, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // T
				{ 0, 1, 0, 0 }, 
				{ 1, 1, 0, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // Z
				{ 1, 1, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 0, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // Z
				{ 0, 0, 1, 0 }, 
				{ 0, 1, 1, 0 }, 
				{ 0, 1, 0, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // Z
				{ 0, 0, 0, 0 }, 
				{ 1, 1, 0, 0 }, 
				{ 0, 1, 1, 0 }, 
			},{ 
				{ 0, 0, 0, 0 }, // Z
				{ 0, 1, 0, 0 }, 
				{ 1, 1, 0, 0 }, 
				{ 1, 0, 0, 0 }, 
			}, 
		};

	private final static int[][] JLSTZ_KICK_TABLE = {
			{0, 0, -1, 0, -1, +1, 0, -2, -1, -2},
			{0, 0, +1, 0, +1, -1, 0, +2, +1, +2},
			{0, 0, +1, 0, +1, -1, 0, +2, +1, +2},
			{0, 0, -1, 0, -1, +1, 0, -2, -1, -2},
			{0, 0, +1, 0, +1, +1, 0, -2, +1, -2},
			{0, 0, -1, 0, -1, -1, 0, +2, -1, +2},
			{0, 0, -1, 0, -1, -1, 0, +2, -1, +2},
			{0, 0, +1, 0, +1, +1, 0, -2, +1, -2},
	};
	
	private final static int[][] I_KICK_TABLE = {
			{0, 0, -2, 0, +1, 0, -2, -1, +1, +2},
			{0, 0, +2, 0, -1, 0, +2, +1, -1, -2},
			{0, 0, -1, 0, +2, 0, -1, +2, +2, -1},
			{0, 0, +1, 0, -2, 0, +1, -2, -2, +1},
			{0, 0, +2, 0, -1, 0, +2, +1, -1, -2},
			{0, 0, -2, 0, +1, 0, -2, -1, +1, +2},
			{0, 0, +1, 0, -2, 0, +1, -2, -2, +1},
			{0, 0, -1, 0, +2, 0, -1, +2, +2, -1},
	};
	// @formatter:on

	public Tetris() {

		board = new int[20];
		bag = generateBag();
		nextBag = generateBag();

		rotation = 0;
		posX = START_X;
		posY = START_Y;
		hold = -1;
	}

	public void gravity() {

		posY++;

		if (lowestY < posY)
			lowestY = posY;
		
		if (hasCollision()) {
			posY--;
			if (currentLockDownPass > LOCK_DOWN_PASSES || lockDownResets > MAX_LOCK_DOWN_RESETS) {
				placePiece();
				resetLockDownPass();
				lockDownResets = 0;
			} else {
				currentLockDownPass++;
			}
		} else {
			currentLockDownPass = 0;
			if (posY > lowestY)
				lockDownResets = 0;
		}
	}

	public void moveLeft() {

		posX--;

		if (hasCollision())
			posX++;
		else
			resetLockDownPass();
	}

	public void moveRight() {

		posX++;

		if (hasCollision())
			posX--;
		else
			resetLockDownPass();
	}
	
	private void resetLockDownPass() {
		currentLockDownPass = 0;
		lockDownResets++;
	}

	public int[] getNextPieces() {
		int[] next = new int[5];
		int p = 0;
		for (int i = currentPiece + 1; i < currentPiece + 1 + next.length; i++) {
			if (i >= bag.length) {
				next[p] = nextBag[i - bag.length];
			} else {
				next[p] = bag[i];
			}
			p++;
		}
		return next;
	}

	public void rotateLeft() {
		int previousRotation = rotation;
		int previousX = posX;
		int previousY = posY;
		rotation--;
		if (rotation < 0)
			rotation += 4;
		final int[] kickTable;
		if (bag[currentPiece] == 0)
			kickTable = I_KICK_TABLE[rotation * 2 + 1];
		else
			kickTable = JLSTZ_KICK_TABLE[rotation * 2 + 1];
		for (int i = 0; i < kickTable.length; i += 2) {
			posX += kickTable[i + 0];
			posY -= kickTable[i + 1];
			if (hasCollision()) {
				posX = previousX;
				posY = previousY;
			} else {
				resetLockDownPass();
				return;
			}
		}
		rotation = previousRotation;
	}

	public void rotateRight() {
		int previousRotation = rotation;
		int previousX = posX;
		int previousY = posY;
		rotation++;
		if (rotation > 3)
			rotation -= 4;
		final int[] kickTable;
		if (bag[currentPiece] == 0)
			kickTable = I_KICK_TABLE[previousRotation * 2];
		else
			kickTable = JLSTZ_KICK_TABLE[previousRotation * 2];
		for (int i = 0; i < kickTable.length; i += 2) {
			posX += kickTable[i + 0];
			posY -= kickTable[i + 1];
			if (hasCollision()) {
				posX = previousX;
				posY = previousY;
			} else {
				resetLockDownPass();
				return;
			}
		}
		rotation = previousRotation;
	}

	public void hold() {
		if (hold == -1) {
			hold = bag[currentPiece];
			nextPiece();
		} else {
			int tmp = hold;
			hold = bag[currentPiece];
			bag[currentPiece] = tmp;
			posX = START_X;
			posY = START_Y;
			rotation = 0;
		}

	}

	public int getHoldPiece() {
		return hold;
	}

	private boolean hasCollision() {
		int[][] piece = getCurrentShape();
		for (int x = 0; x < piece.length; x++) {
			for (int y = posY; y < posY + piece[0].length; y++) {
				if (piece[y - posY][x] == 0)
					continue;
				if (posX + x < 0 || posX + x > 9)
					return true;
				if (y < 0)
					continue;
				if (y >= 20 || (board[y] & (0b1000000000 >> posX + x)) > 0)
					return true;
			}
		}
		return false;
	}

	private void placePiece() {
		placePieceOnBoard(board);
		clearLines();
		nextPiece();
		lowestY = 0;
	}

	private void clearLines() {
		
		boolean isTSpin = false;
		if (bag[currentPiece] == 5) {
			int cornersWithBlock = 0;
			if (posX + 0 < 0 || (board[posY + 1] & (0b1000000000 >> (posX + 0))) > 0)
				cornersWithBlock++;
			if (posX + 2 > 9 || (board[posY + 1] & (0b1000000000 >> (posX + 2))) > 0)
				cornersWithBlock++;
			if (posY + 3 >= board.length || posX + 0 < 0 || (board[posY + 3] & (0b1000000000 >> (posX + 0))) > 0)
				cornersWithBlock++;
			if (posY + 3 >= board.length || posX + 2 > 9 || (board[posY + 3] & (0b1000000000 >> (posX + 2))) > 0)
				cornersWithBlock++;
			if (cornersWithBlock > 2)
				isTSpin = true;
		}
		
		int linesCleared = 0;
		for (int i = 0; i < board.length; i++) {
			if ((board[i] ^ 0b1111111111) == 0) {
				for (int j = i; j >= 0; j--) {
					if (j == 0)
						board[j] = 0;
					else
						board[j] = board[j - 1];
				}
				linesCleared++;
			}
		}
		
		switch (linesCleared) {
		case 1:
			if (isTSpin)
				totalLinesSent += 2;
			break;
		case 2:
			if (isTSpin) 
				totalLinesSent += 4;
			else
				totalLinesSent += 1;
			break;
		case 3:
			if (isTSpin)
				totalLinesSent += 6;
			else
				totalLinesSent += 2;
			break;
		case 4:
			totalLinesSent += 4;
			break;
		}
		int totalBoard = 0;
		for (int l : board) {
			totalBoard += l;
		}
		if (totalBoard == 0) {
			totalLinesSent += 10;
		}
	}
	
	public int getTotalLinesSent() {
		return totalLinesSent;
	}

	private void nextPiece() {
		currentPiece++;

		if (currentPiece == 7) {
			bag = nextBag;
			nextBag = generateBag();
			currentPiece = 0;
		}

		posX = START_X;
		posY = START_Y;
		rotation = 0;
	}

	public void hardDrop() {
		while (!hasCollision()) {
			posY++;
		}
		posY--;
		placePiece();
		resetLockDownPass();
		lockDownResets = 0;
	}

	private int[] generateBag() {
		int[] newBag = new int[7];
		for (int i = 0; i < newBag.length; i++) {
			newBag[i] = i;
		}
		for (int i = 0; i < newBag.length; i++) {
			int random = (int) (Math.random() * newBag.length);
			int tmp = newBag[i];
			newBag[i] = newBag[random];
			newBag[random] = tmp;
		}
		return newBag;
	}

	public int[] getBoard() {
		int[] b = board.clone();
		placePieceOnBoard(b);
		return b;
	}

	private void placePieceOnBoard(int[] board) {
		int[][] piece = getCurrentShape();

		for (int x = 0; x < piece.length; x++) {
			for (int y = posY; y < posY + piece[0].length; y++) {
				if (y < 0)
					continue;
				if (piece[y - posY][x] == 0)
					continue;
				board[y] = board[y] | (0b1000000000 >> (posX + x));
			}
		}
	}

	private int[][] getCurrentShape() {
		return TETROMINOS[bag[currentPiece] * 4 + rotation];
	}

	public int[][] getPieceShape(int piece) {
		return TETROMINOS[piece * 4];
	}
	
	public int getCurrentPiece() {
		return bag[currentPiece];
	}
}
