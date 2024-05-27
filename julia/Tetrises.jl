module Tetrises

export Tetris,
gravity,
move_left,
move_right,
rotate_left,
rotate_right,
hold,
get_next_pieces,
get_hold_piece,
get_total_lines_sent,
hard_drop,
soft_drop,
get_board,
get_piece_shape,
get_current_piece,
is_game_over,
get_score

const TETROMINOS = [ 
    [ 
        [ 0, 0, 0, 0 ], # I
        [ 1, 1, 1, 1 ], 
        [ 0, 0, 0, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 1, 0 ], # I
        [ 0, 0, 1, 0 ], 
        [ 0, 0, 1, 0 ], 
        [ 0, 0, 1, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # I
        [ 0, 0, 0, 0 ], 
        [ 1, 1, 1, 1 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 1, 0, 0 ], # I
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # J
        [ 1, 0, 0, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # J
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # J
        [ 0, 0, 0, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 0, 0, 1, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # J
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
        [ 1, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # L
        [ 0, 0, 1, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # L
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # L
        [ 0, 0, 0, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 1, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # L
        [ 1, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # O
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # O
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # O
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # O
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # S
        [ 0, 1, 1, 0 ], 
        [ 1, 1, 0, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # S
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 1, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # S
        [ 0, 0, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 1, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # S
        [ 1, 0, 0, 0 ], 
        [ 1, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # T
        [ 0, 1, 0, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # T
        [ 0, 1, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # T
        [ 0, 0, 0, 0 ], 
        [ 1, 1, 1, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # T
        [ 0, 1, 0, 0 ], 
        [ 1, 1, 0, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # Z
        [ 1, 1, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 0, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # Z
        [ 0, 0, 1, 0 ], 
        [ 0, 1, 1, 0 ], 
        [ 0, 1, 0, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # Z
        [ 0, 0, 0, 0 ], 
        [ 1, 1, 0, 0 ], 
        [ 0, 1, 1, 0 ], 
    ],[ 
        [ 0, 0, 0, 0 ], # Z
        [ 0, 1, 0, 0 ], 
        [ 1, 1, 0, 0 ], 
        [ 1, 0, 0, 0 ], 
    ], 
]

const JLSTZ_KICK_TABLE = [
    [0, 0, -1, 0, -1, +1, 0, -2, -1, -2],
    [0, 0, +1, 0, +1, -1, 0, +2, +1, +2],
    [0, 0, +1, 0, +1, -1, 0, +2, +1, +2],
    [0, 0, -1, 0, -1, +1, 0, -2, -1, -2],
    [0, 0, +1, 0, +1, +1, 0, -2, +1, -2],
    [0, 0, -1, 0, -1, -1, 0, +2, -1, +2],
    [0, 0, -1, 0, -1, -1, 0, +2, -1, +2],
    [0, 0, +1, 0, +1, +1, 0, -2, +1, -2],
]

const I_KICK_TABLE = [
    [0, 0, -2, 0, +1, 0, -2, -1, +1, +2],
    [0, 0, +2, 0, -1, 0, +2, +1, -1, -2],
    [0, 0, -1, 0, +2, 0, -1, +2, +2, -1],
    [0, 0, +1, 0, -2, 0, +1, -2, -2, +1],
    [0, 0, +2, 0, -1, 0, +2, +1, -1, -2],
    [0, 0, -2, 0, +1, 0, -2, -1, +1, +2],
    [0, 0, +1, 0, -2, 0, +1, -2, -2, +1],
    [0, 0, -1, 0, +2, 0, -1, +2, +2, -1],
]

const START_X = 3
const START_Y = 1
const LOCK_DOWN_PASSES = 4
const MAX_LOCK_DOWN_RESETS = 15

mutable struct Tetris

    board::Vector{Int16}
    current_piece::Int8
    rotation::Int8
    bag::Vector{Int8}
    next_bag::Vector{Int8}
    pos_x::Int8
    pos_y::Int8
    current_lock_down_pass::Int8
    hold::Int8
    lock_down_resets::Int8
    total_lines_sent::Int
    lowest_y::Int8
    game_over::Bool
    can_hold::Bool
    score::Int

    function Tetris()
        new(zeros(Int8, 24), 1, 0, _generate_bag(), _generate_bag(), START_X, START_Y, 0, -1, 0, 0, 1, false, true)
    end
end

function _reset_lock_down_pass(tetris::Tetris)
    tetris.current_lock_down_pass = 0
    tetris.lock_down_resets += 1
end

function _has_collision(tetris::Tetris)
    piece = _get_current_shape(tetris)
    for x in 1:length(piece)
        for y in tetris.pos_y + 1:(tetris.pos_y + length(piece[1]) - 1)
            if piece[y - tetris.pos_y + 1][x] == 0
                continue
            end
            if tetris.pos_x + x < 1 || tetris.pos_x + x > 10
                return true
            end
            if y >= 24 || (tetris.board[y] & (0b1000000000 >> (tetris.pos_x + x - 1))) > 0
                return true
            end
        end
    end
    return false
end

function _place_piece(tetris::Tetris)
    _place_piece_on_board(tetris, tetris.board)
    _clear_lines(tetris)
    _next_piece(tetris)
    tetris.lowest_y = 0
    tetris.can_hold = true
end

function _clear_lines(tetris::Tetris)
    is_t_spin = false
    if tetris.bag[tetris.current_piece] == 5
        corners_with_block = 0
        if tetris.pos_x + 0 < 1 || (tetris.pos_y + 1 >= 1 && (tetris.board[tetris.pos_y + 1] & (0b1000000000 >> (tetris.pos_x + 0 - 1))) > 0)
            corners_with_block += 1
        end
        if tetris.pos_x + 2 > 10 || (tetris.pos_y + 1 >= 1 && (tetris.board[tetris.pos_y + 1] & (0b1000000000 >> (tetris.pos_x + 2 - 1))) > 0)
            corners_with_block += 1
        end
        if tetris.pos_y + 3 >= length(tetris.board) || tetris.pos_x + 0 < 1 || (tetris.pos_y + 3 >= 1 && (tetris.board[tetris.pos_y + 3] & (0b1000000000 >> (tetris.pos_x + 0 - 1))) > 0)
            corners_with_block += 1
        end
        if tetris.pos_y + 3 >= length(tetris.board) || tetris.pos_x + 2 > 10 || (tetris.pos_y + 3 >= 1 && (tetris.board[tetris.pos_y + 3] & (0b1000000000 >> (tetris.pos_x + 2 - 1))) > 0)
            corners_with_block += 1
        end
        if corners_with_block > 2
            is_t_spin = true
        end
    end
    lines_cleared = 0
    for i in 1:length(tetris.board)
        if (tetris.board[i] ⊻ 0b1111111111) == 0
            for j in i:-1:1
                if j == 1
                    tetris.board[j] = 0
                else
                    tetris.board[j] = tetris.board[j - 1]
                end
            end
            lines_cleared += 1
        end
    end

    score = 0
    is_perfect_clear = false

    total_board = sum(tetris.board)
    if total_board == 0
        tetris.total_lines_sent += 10
        is_perfect_clear = true
    end

    if lines_cleared == 0
        if is_t_spin
            score += 400
        end
    elseif lines_cleared == 1
        if is_t_spin
            tetris.total_lines_sent += 2
            score += 800
        else
            score += 100
            if is_perfect_clear
                score += 800
            end
        end
    elseif lines_cleared == 2
        if is_t_spin
            tetris.total_lines_sent += 4
            score += 1200
        else
            tetris.total_lines_sent += 1
            score += 300
            if is_perfect_clear
                score += 1200
            end
        end
    elseif lines_cleared == 3
        if is_t_spin
            tetris.total_lines_sent += 6
            score += 1600
        else
            tetris.total_lines_sent += 2
            score += 500
            if is_perfect_clear
                score += 1800
            end
        end
    elseif lines_cleared == 4
        tetris.total_lines_sent += 4
        score += 800
        if is_perfect_clear
            score += 2000
        end
    end

end

function _next_piece(tetris::Tetris)
    tetris.current_piece += 1
    if tetris.current_piece == 8
        tetris.bag = tetris.next_bag
        tetris.next_bag = _generate_bag()
        tetris.current_piece = 1
    end
    tetris.pos_x = START_X
    tetris.pos_y = START_Y
    tetris.rotation = 0
    if _has_collision(tetris)
        tetris.game_over = true
    end
end

function _generate_bag()
    new_bag::Vector{Int8} = collect(1:7)
    for i in eachindex(new_bag)
        random_index = rand(1:length(new_bag))
        new_bag[i], new_bag[random_index] = new_bag[random_index], new_bag[i]
    end
    return new_bag
end

function _place_piece_on_board(tetris::Tetris, board)
    piece = _get_current_shape(tetris)
    for x in 1:length(piece)
        for y in tetris.pos_y:(tetris.pos_y + length(piece[1]) - 1)
            if y < 1
                continue
            end
            if piece[y - tetris.pos_y + 1][x] == 0
                continue
            end
            board[y] |= (0b1000000000 >> (tetris.pos_x + x - 1))
        end
    end
end

function _get_current_shape(tetris::Tetris)
    return TETROMINOS[tetris.bag[tetris.current_piece] * 4 - 4 + tetris.rotation + 1]
end

function gravity(tetris::Tetris)
    tetris.pos_y += 1
    if tetris.lowest_y < tetris.pos_y
        tetris.lowest_y = tetris.pos_y
    end
    if _has_collision(tetris)
        tetris.pos_y -= 1
        if tetris.current_lock_down_pass > 15 || tetris.lock_down_resets > 20
            _place_piece(tetris)
            _reset_lock_down_pass(tetris)
            tetris.lock_down_resets = 0
        else
            tetris.current_lock_down_pass += 1
        end
    else
        tetris.current_lock_down_pass = 0
        if tetris.pos_y > tetris.lowest_y
            tetris.lock_down_resets = 0
        end
    end
end

function move_left(tetris::Tetris)
    tetris.pos_x -= 1
    if _has_collision(tetris)
        tetris.pos_x += 1
    else
        _reset_lock_down_pass(tetris)
    end
end

function move_right(tetris::Tetris)
    tetris.pos_x += 1
    if _has_collision(tetris)
        tetris.pos_x -= 1
    else
        _reset_lock_down_pass(tetris)
    end
end

function get_next_pieces(tetris::Tetris)
    next_pieces = zeros(Int, 5)
    p = 1
    for i in tetris.current_piece + 1 : tetris.current_piece + 1 + 4
        if i >= length(tetris.bag)
            next_pieces[p] = tetris.next_bag[i - length(tetris.bag) + 1]
        else
            next_pieces[p] = tetris.bag[i]
        end
        p += 1
    end
    return next_pieces
end

function rotate_left(tetris::Tetris)
    previous_rotation = tetris.rotation
    previous_x = tetris.pos_x
    previous_y = tetris.pos_y
    tetris.rotation -= 1
    if tetris.rotation < 0
        tetris.rotation += 4
    end
    kick_table = I_KICK_TABLE[tetris.rotation * 2 + 2] * (tetris.bag[tetris.current_piece] == 0) + JLSTZ_KICK_TABLE[tetris.rotation * 2 + 2] * (tetris.bag[tetris.current_piece] != 0)
    for i in 1:2:length(kick_table)
        tetris.pos_x += kick_table[i]
        tetris.pos_y -= kick_table[i+1]
        if _has_collision(tetris)
            tetris.pos_x = previous_x
            tetris.pos_y = previous_y
        else
            _reset_lock_down_pass(tetris)
            return
        end
    end
    tetris.rotation = previous_rotation
end

function rotate_right(tetris::Tetris)
    previous_rotation = tetris.rotation
    previous_x = tetris.pos_x
    previous_y = tetris.pos_y
    tetris.rotation += 1
    if tetris.rotation > 3
        tetris.rotation -= 4
    end
    kick_table = I_KICK_TABLE[previous_rotation*2 + 1] * (tetris.bag[tetris.current_piece] == 0) + JLSTZ_KICK_TABLE[previous_rotation*2 + 1] * (tetris.bag[tetris.current_piece] != 0)
    for i in 1:2:length(kick_table)
        tetris.pos_x += kick_table[i]
        tetris.pos_y -= kick_table[i+1]
        if _has_collision(tetris)
            tetris.pos_x = previous_x
            tetris.pos_y = previous_y
        else
            _reset_lock_down_pass(tetris)
            return
        end
    end
    tetris.rotation = previous_rotation
end

function hold(tetris::Tetris)
    if !tetris.can_hold
        return
    end
    if tetris.hold == -1
        tetris.hold = tetris.bag[tetris.current_piece]
        _next_piece(tetris)
    else
        tetris.hold, tetris.bag[tetris.current_piece] = tetris.bag[tetris.current_piece], tetris.hold
        tetris.pos_x = 4
        tetris.pos_y = 0
        tetris.rotation = 0
    end
    tetris.can_hold = false
end

function get_hold_piece(tetris::Tetris)
    return tetris.hold
end

function get_total_lines_sent(tetris::Tetris)
    return tetris.total_lines_sent
end

function hard_drop(tetris::Tetris)
    while !_has_collision(tetris)
        tetris.pos_y += 1
        tetris.score += 2
    end
    tetris.pos_y -= 1
    tetris.score -= 2
    _place_piece(tetris)
    _reset_lock_down_pass(tetris)
    tetris.lock_down_resets = 0
end

function soft_drop(tetris::Tetris)
    while !_has_collision(tetris)
        tetris.pos_y += 1
        tetris.score += 1
    end
    tetris.pos_y -= 1
    tetris.score -= 1
end

function get_board(tetris::Tetris)
    b = copy(tetris.board)
    _place_piece_on_board(tetris, b)
    return b
end

function get_piece_shape(piece)
    return TETROMINOS[piece * 4 - 4 + 1]
end

function get_current_piece(tetris::Tetris)
    return tetris.bag[tetris.current_piece + 1]
end

function is_game_over(tetris::Tetris)
    return tetris.game_over
end

function get_score(tetris::Tetris)
    return tetris.score
end

end