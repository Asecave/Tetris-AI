package com.asecave;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;

import javax.swing.JFrame;
import javax.swing.JPanel;

@SuppressWarnings("serial")
class Main extends JPanel implements KeyListener {

	JFrame frame;

	final int blockSize = 50;

	Tetris tetris = new Tetris();

	public Main() {

		frame = new JFrame("AI Tetris");
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

		this.setPreferredSize(new Dimension(blockSize * 22, blockSize * 22));
		frame.add(this);
		frame.pack();
		frame.setLocationRelativeTo(null);
		frame.addKeyListener(this);
		frame.setVisible(true);

		while (true) {
			tetris.gravity();
			frame.repaint();
			try {
				Thread.sleep(100);
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}

//		for (int i = 0; i < 100; i++) {
//			tetris.gravity();
//			tetris.roatateLeft();
//
//			int[] board = tetris.getBoard();
//
//			System.out.println(Arrays.toString(tetris.getNextPieces()));
//			for (int j = 0; j < board.length; j++) {
//				System.out.println(String.format("%10s", Integer.toBinaryString(board[j] & 0xFFFF)).replace(' ', '0')
//						.replaceAll("0", "."));
//			}
//			System.out.println();
//
//			try {
//				Thread.sleep(200);
//			} catch (InterruptedException e) {
//				e.printStackTrace();
//			}
//		}
	}

	@Override
	public void paint(Graphics g) {
		super.paint(g);
		Graphics2D g2d = (Graphics2D) g;

		g2d.setColor(Color.DARK_GRAY);
		g2d.fillRect(0, 0, frame.getWidth(), frame.getHeight());
		
		g2d.setColor(Color.GREEN);
		if (tetris.getHoldPiece() != -1) {
			int[][] shape = tetris.getPieceShape(tetris.getHoldPiece());
			for (int x = 0; x < shape.length; x++) {
				for (int y = 0; y < shape[0].length; y++) {
					if (shape[y][x] != 1)
						continue;
					g2d.fill3DRect(x * blockSize + blockSize / 2, y * blockSize - blockSize / 2, blockSize, blockSize, true);
				}
			}
		}
		
		int[] nextPieces = tetris.getNextPieces();
		for(int i = 0; i < nextPieces.length; i++) {
			int[][] shape = tetris.getPieceShape(nextPieces[i]);
			for (int x = 0; x < shape.length; x++) {
				for (int y = 0; y < shape[0].length; y++) {
					if (shape[y][x] != 1)
						continue;
					g2d.fill3DRect((x + 17) * blockSize + blockSize / 2, y * blockSize - blockSize / 2 + (i * 3 * blockSize), blockSize, blockSize, true);
				}
			}
		}

		g2d.translate(blockSize * 6, blockSize);

		g2d.setColor(new Color(Color.DARK_GRAY.getRGB()).darker());
		g2d.fillRect(-blockSize, -blockSize, blockSize * 12, blockSize * 22);
		int[] board = tetris.getBoard();
		for (int i = 0; i < board.length; i++) {
			for (int j = 0; j < 10; j++) {
				int current = 0b1000000000 >> j;
				if ((current & board[i]) > 0) {
					g2d.setColor(Color.GREEN);
					g2d.fill3DRect(j * blockSize, i * blockSize, blockSize, blockSize, true);
				} else {
					g2d.setColor(Color.DARK_GRAY);
					g2d.fill3DRect(j * blockSize, i * blockSize, blockSize, blockSize, true);
				}
			}
		}
	}

	public static void main(String[] args) {
		new Main();
	}

	@Override
	public void keyTyped(KeyEvent e) {

	}

	@Override
	public void keyPressed(KeyEvent e) {
		switch (e.getKeyCode()) {
		case KeyEvent.VK_ESCAPE:
			System.exit(0);
		case KeyEvent.VK_A:
			tetris.moveLeft();
			break;
		case KeyEvent.VK_D:
			tetris.moveRight();
			break;
		case KeyEvent.VK_LEFT:
			tetris.rotateLeft();
			break;
		case KeyEvent.VK_RIGHT:
			tetris.rotateRight();
			break;
		case KeyEvent.VK_UP:
			tetris.hardDrop();
			break;
		case KeyEvent.VK_SHIFT:
			tetris.hold();
			break;
		}
	}

	@Override
	public void keyReleased(KeyEvent e) {

	}

}