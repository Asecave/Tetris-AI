package com.asecave;

public class SynchronizedTetris extends Tetris {

	@Override
	public synchronized void gravity() {
		super.gravity();
	}

	@Override
	public synchronized void moveLeft() {
		super.moveLeft();
	}

	@Override
	public synchronized void moveRight() {
		super.moveRight();
	}

	@Override
	public synchronized int[] getNextPieces() {
		return super.getNextPieces();
	}

	@Override
	public synchronized void rotateLeft() {
		super.rotateLeft();
	}

	@Override
	public synchronized void rotateRight() {
		super.rotateRight();
	}

	@Override
	public synchronized void hold() {
		super.hold();
	}

	@Override
	public synchronized int getHoldPiece() {
		return super.getHoldPiece();
	}

	@Override
	public synchronized int getTotalLinesSent() {
		return super.getTotalLinesSent();
	}

	@Override
	public synchronized void hardDrop() {
		super.hardDrop();
	}

	@Override
	public synchronized int[] getBoard() {
		return super.getBoard();
	}

	@Override
	public synchronized int[][] getPieceShape(int piece) {
		return super.getPieceShape(piece);
	}

	@Override
	public synchronized int getCurrentPiece() {
		return super.getCurrentPiece();
	}

}
