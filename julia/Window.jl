using GLMakie

BLOCK_SIZE = 30
BLOCKS_X = 22
BLOCKS_Y = 22

scene = Scene(size = (BLOCK_SIZE * BLOCKS_X, BLOCK_SIZE * BLOCKS_Y), title = "Tetris AI")

function window()

    draw_window()

    on(events(scene).keyboardbutton) do event
        if event.action in (Keyboard.press, Keyboard.repeat)
            event.key == Keyboard.d && move_right(tetris)
            event.key == Keyboard.a && move_left(tetris)
            event.key == Keyboard.left && rotate_left(tetris)
            event.key == Keyboard.right && rotate_right(tetris)
            event.key == Keyboard.up && hard_drop(tetris)
            event.key == Keyboard.down && soft_drop(tetris)
            event.key == Keyboard.left_shift && hold(tetris)
            event.key == Keyboard.escape && exit()
        end
    end

    on(events(scene).window_open) do open
        if (!open)
            exit()
        end
    end

    while true
        gravity(tetris)
        if is_game_over(tetris)
            exit()
        end

        draw_window()
        sleep(1/60)
    end
end

function draw_window()

    # empty!(scene)
    draw_square!(scene, 0, 0, 22, :darkgray)

    for i in 0:4
        draw_square!(scene, 0, i, 1, :gray)
        draw_square!(scene, i, 5, 1, :gray)
        draw_square!(scene, i + 16, 15, 1, :gray)
    end

    for i in 0 : 21
        draw_square!(scene, i, 0, 1, :gray)
    end

    for i in 0:20
        draw_square!(scene, 5, i + 1, 1, :gray)
        draw_square!(scene, 16, i + 1, 1, :gray)
    end

    for i in 0:9
        draw_square!(scene, i + 6, 21, 1, :gray)
    end

    for i in 0:14
        draw_square!(scene, 21, i + 1, 1, :gray)
    end

    if get_hold_piece(tetris) != -1
        draw_piece(scene, get_piece_shape(get_hold_piece(tetris)), 1, 1)
    end

    for i in 4:length(get_board(tetris))
        line = get_board(tetris)[i]
        for j in 0:9
            current = 0b1000000000 >>> j
            if (current & line) > 0
                draw_square!(scene, j + 6, i - 3, 1, :cyan)
            end
        end
    end

    for i in 1:length(get_next_pieces(tetris))
        draw_piece(scene, get_piece_shape(get_next_pieces(tetris)[i]), 17, 1 + i * 3 - 4)
    end

    display(scene)
end

function draw_square!(scene, x, y, size, color)
    x /= BLOCKS_X / 2
    y /= BLOCKS_Y / 2
    x -= 1
    y -= 1
    y *= -1
    size /= BLOCKS_X / 2
    points = Point2f[
        (x, y),
        (x + size, y),
        (x + size, y - size),
        (x, y - size)
        ]
    poly!(scene, points, color=color)
end

function draw_piece(scene, piece, offset_x, offset_y)

    for y in 1:4
        for x in 1:4
            if piece[y][x] != 0
                draw_square!(scene, x + offset_x - 1, y + offset_y - 1, 1, :lime)
            end
        end
    end
end