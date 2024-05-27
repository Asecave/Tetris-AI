module ConsoleUIs

using ..Tetrises

export draw

const BACKGROUND = "  "
const PIECE = "██"

const WALL_COLOR = (120, 120, 120)
const BACKGROUND_COLOR = (30, 30, 30)
const PIECE_COLOR = (50, 150, 150)

function draw(tetris::Tetris)

    board = [
        split(" ┌`─ `HO`LD` ─`┐┌`──`──`──` T`ET`RI`S `──`──`──`┐┌`─ `NE`XT` ─`┐ ", "`"),
        split(" │`  `  `  `  `││`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split(" │`  `  `  `  `││`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split(" │`  `  `  `  `││`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split(" │`  `  `  `  `││`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split(" └`──`──`──`──`┘│`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `││`  `  `  `  `│ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `│└`──`──`──`──`┘ ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `│ `  `  `  `  `  ", "`"),
        split("  `LI`NE`S `  ` │`  `  `  `  `  `  `  `  `  `  `│ `  `  `  `  `  ", "`"),
        split("  `SE`NT`: `  ` │`  `  `  `  `  `  `  `  `  `  `│ `  `SC`OR`E:`  ", "`"),
        split("  `  `  `  `  ` │`  `  `  `  `  `  `  `  `  `  `│ `  `  `  `  `  ", "`"),
        split("  `  `  `  `  ` └`──`──`──`──`──`──`──`──`──`──`┘ `  `  `  `  `  ", "`"),
    ]

    board = transpose(board)

    for x in eachindex(board)
        for y in eachindex(board[x])
            board[x][y] = get_ansi_fbrgb(WALL_COLOR, BACKGROUND_COLOR, board[x][y])
        end
    end

    if get_hold_piece(tetris) != -1
        _draw_piece(board, get_piece_shape(get_hold_piece(tetris)), 1, 1)
    end

    for i in 4:length(get_board(tetris))
        line = get_board(tetris)[i]
        for j in 0:9
            current = 0b1000000000 >>> j
            if (current & line) > 0
                board[j + 7][i - 2] = PIECE
            end
        end
    end

    for i in 1:length(get_next_pieces(tetris))
        _draw_piece(board, get_piece_shape(get_next_pieces(tetris)[i]), 18, 1 + i * 3 - 2)
    end


    text_color = (100, 255, 255)

    x = 18
    y = 21
    score = ("" * lpad(get_score(tetris), 6, "0"))
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, score[1:2], false), true)
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, score[3:4], false), true)
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, score[5:6], false), true)

    x = 1
    y = 21
    lines_sent = ("" * lpad(get_total_lines_sent(tetris), 6, "0"))
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, lines_sent[1:2], false), true)
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, lines_sent[3:4], false), true)
    board[x+=1][y] = get_ansi_rgb(BACKGROUND_COLOR, get_ansi_rgb(text_color, lines_sent[5:6], false), true)

    board = transpose(board)

    for row in board
        for cell in row
            print(cell)
        end
        println()
    end
end

function transpose(board)
    for i in eachindex(board)
        for j in 1:i
            board[i][j], board[j][i] = board[j][i], board[i][j]
        end
    end
    return board
end

function _draw_piece(board, shape, offset_x, offset_y)
    for y in 1:4
        for x in 1:4
            if shape[y][x] != 0
                board[x + offset_x - 1][y + offset_y - 1] = get_ansi_rgb(PIECE_COLOR, PIECE, false)
            end
        end
    end
end

function get_ansi_fbrgb((fgr, fgg, fgb), (bgr, bgg, bgb), text)
    return get_ansi_rgb((fgr, fgg, fgb), get_ansi_rgb((bgr, bgg, bgb), text, true), false)
end

function get_ansi_rgb((r, g, b), text, background)
    bg = ""
    if background
        bg = "48"
    else
        bg = "38"
    end
    return "\e[1m\e[" * bg * ";2;$r;$g;$b;249m" * text * "\e[0m"
end

end