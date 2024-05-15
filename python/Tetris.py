import random

class Tetris:
    TETROMINOS = [ 
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

    JLSTZ_KICK_TABLE = [
            [0, 0, -1, 0, -1, +1, 0, -2, -1, -2],
            [0, 0, +1, 0, +1, -1, 0, +2, +1, +2],
            [0, 0, +1, 0, +1, -1, 0, +2, +1, +2],
            [0, 0, -1, 0, -1, +1, 0, -2, -1, -2],
            [0, 0, +1, 0, +1, +1, 0, -2, +1, -2],
            [0, 0, -1, 0, -1, -1, 0, +2, -1, +2],
            [0, 0, -1, 0, -1, -1, 0, +2, -1, +2],
            [0, 0, +1, 0, +1, +1, 0, -2, +1, -2],
    ]
    
    I_KICK_TABLE = [
            [0, 0, -2, 0, +1, 0, -2, -1, +1, +2],
            [0, 0, +2, 0, -1, 0, +2, +1, -1, -2],
            [0, 0, -1, 0, +2, 0, -1, +2, +2, -1],
            [0, 0, +1, 0, -2, 0, +1, -2, -2, +1],
            [0, 0, +2, 0, -1, 0, +2, +1, -1, -2],
            [0, 0, -2, 0, +1, 0, -2, -1, +1, +2],
            [0, 0, +1, 0, -2, 0, +1, -2, -2, +1],
            [0, 0, -1, 0, +2, 0, -1, +2, +2, -1],
    ]

    def __init__(self):

        START_X = 3
        START_Y = 0
        LOCK_DOWN_PASSES = 4
        MAX_LOCK_DOWN_RESETS = 15

        self.board = [0] * 24
        self.current_piece = 0
        self.rotation = 0
        self.bag = []
        self.next_bag = []
        self.pos_x = START_X
        self.pos_y = START_Y
        self.current_lock_down_pass = 0
        self.hold = -1
        self.lock_down_resets = 0
        self.total_lines_sent = 0
        self.lowest_y = 0
        self.game_over = False
        self.can_hold = True

        

        self.board = [0] * 24
        self.bag = self.generate_bag()
        self.next_bag = self.generate_bag()
        self.rotation = 0
        self.pos_x = 4
        self.pos_y = 0
        self.hold = -1
        self.game_over = False
        self.can_hold = True

    def gravity(self):
        self.pos_y += 1
        if self.lowest_y < self.pos_y:
            self.lowest_y = self.pos_y
        if self.has_collision():
            self.pos_y -= 1
            if self.current_lock_down_pass > 15 or self.lock_down_resets > 20:
                self.place_piece()
                self.reset_lock_down_pass()
                self.lock_down_resets = 0
            else:
                self.current_lock_down_pass += 1
        else:
            self.current_lock_down_pass = 0
            if self.pos_y > self.lowest_y:
                self.lock_down_resets = 0

    def move_left(self):
        self.pos_x -= 1
        if self.has_collision():
            self.pos_x += 1
        else:
            self.reset_lock_down_pass()

    def move_right(self):
        self.pos_x += 1
        if self.has_collision():
            self.pos_x -= 1
        else:
            self.reset_lock_down_pass()

    def reset_lock_down_pass(self):
        self.current_lock_down_pass = 0
        self.lock_down_resets += 1

    def get_next_pieces(self):
        next_pieces = [0] * 5
        p = 0
        for i in range(self.current_piece + 1, self.current_piece + 1 + 5):
            if i >= len(self.bag):
                next_pieces[p] = self.next_bag[i - len(self.bag)]
            else:
                next_pieces[p] = self.bag[i]
            p += 1
        return next_pieces

    def rotate_left(self):
        previous_rotation = self.rotation
        previous_x = self.pos_x
        previous_y = self.pos_y
        self.rotation -= 1
        if self.rotation < 0:
            self.rotation += 4
        kick_table = self.I_KICK_TABLE[self.rotation * 2 + 1] if self.bag[self.current_piece] == 0 else self.JLSTZ_KICK_TABLE[self.rotation * 2 + 1]
        for i in range(0, len(kick_table), 2):
            self.pos_x += kick_table[i]
            self.pos_y -= kick_table[i + 1]
            if self.has_collision():
                self.pos_x = previous_x
                self.pos_y = previous_y
            else:
                self.reset_lock_down_pass()
                return
        self.rotation = previous_rotation

    def rotate_right(self):
        previous_rotation = self.rotation
        previous_x = self.pos_x
        previous_y = self.pos_y
        self.rotation += 1
        if self.rotation > 3:
            self.rotation -= 4
        kick_table = self.I_KICK_TABLE[previous_rotation * 2] if self.bag[self.current_piece] == 0 else self.JLSTZ_KICK_TABLE[previous_rotation * 2]
        for i in range(0, len(kick_table), 2):
            self.pos_x += kick_table[i]
            self.pos_y -= kick_table[i + 1]
            if self.has_collision():
                self.pos_x = previous_x
                self.pos_y = previous_y
            else:
                self.reset_lock_down_pass()
                return
        self.rotation = previous_rotation

    def hold(self):
        if not self.can_hold:
            return
        if self.hold == -1:
            self.hold = self.bag[self.current_piece]
            self.next_piece()
        else:
            self.hold, self.bag[self.current_piece] = self.bag[self.current_piece], self.hold
            self.pos_x = 4
            self.pos_y = 0
            self.rotation = 0
        self.can_hold = False

    def get_hold_piece(self):
        return self.hold

    def has_collision(self):
        piece = self.get_current_shape()
        for x in range(len(piece)):
            for y in range(self.pos_y, self.pos_y + len(piece[0])):
                if piece[y - self.pos_y][x] == 0:
                    continue
                if self.pos_x + x < 0 or self.pos_x + x > 9:
                    return True
                if y >= 24 or (self.board[y] & (0b1000000000 >> (self.pos_x + x))) > 0:
                    return True
        return False

    def place_piece(self):
        self.place_piece_on_board(self.board)
        self.clear_lines()
        self.next_piece()
        self.lowest_y = 0
        self.can_hold = True

    def clear_lines(self):
        is_t_spin = False
        if self.bag[self.current_piece] == 5:
            corners_with_block = 0
            if self.pos_x + 0 < 0 or (self.pos_y + 1 >= 0 and (self.board[self.pos_y + 1] & (0b1000000000 >> (self.pos_x + 0))) > 0):
                corners_with_block += 1
            if self.pos_x + 2 > 9 or (self.pos_y + 1 >= 0 and (self.board[self.pos_y + 1] & (0b1000000000 >> (self.pos_x + 2))) > 0):
                corners_with_block += 1
            if self.pos_y + 3 >= len(self.board) or self.pos_x + 0 < 0 or (self.pos_y + 3 >= 0 and (self.board[self.pos_y + 3] & (0b1000000000 >> (self.pos_x + 0))) > 0):
                corners_with_block += 1
            if self.pos_y + 3 >= len(self.board) or self.pos_x + 2 > 9 or (self.pos_y + 3 >= 0 and (self.board[self.pos_y + 3] & (0b1000000000 >> (self.pos_x + 2))) > 0):
                corners_with_block += 1
            if corners_with_block > 2:
                is_t_spin = True

        lines_cleared = 0
        for i in range(len(self.board)):
            if (self.board[i] ^ 0b1111111111) == 0:
                for j in range(i, -1, -1):
                    if j == 0:
                        self.board[j] = 0
                    else:
                        self.board[j] = self.board[j - 1]
                lines_cleared += 1

        match lines_cleared:
            case 1:
                if is_t_spin:
                    self.total_lines_sent += 2
            case 2:
                if is_t_spin:
                    self.total_lines_sent += 4
                else:
                    self.total_lines_sent += 1
            case 3:
                if is_t_spin:
                    self.total_lines_sent += 6
                else:
                    self.total_lines_sent += 2
            case 4:
                self.total_lines_sent += 4

        total_board = sum(self.board)
        if total_board == 0:
            self.total_lines_sent += 10

    def get_total_lines_sent(self):
        return self.total_lines_sent

    def next_piece(self):
        self.current_piece += 1

        if self.current_piece == 7:
            self.bag = self.next_bag
            self.next_bag = self.generate_bag()
            self.current_piece = 0

        self.pos_x = self.START_X
        self.pos_y = self.START_Y
        self.rotation = 0

        if self.has_collision():
            self.game_over = True

    def hard_drop(self):
        while not self.has_collision():
            self.pos_y += 1
        self.pos_y -= 1
        self.place_piece()
        self.reset_lock_down_pass()
        self.lock_down_resets = 0

    def generate_bag(self):
        new_bag = list(range(7))
        for i in range(len(new_bag)):
            random_index = random.randint(0, len(new_bag) - 1)
            new_bag[i], new_bag[random_index] = new_bag[random_index], new_bag[i]
        return new_bag

    def get_board(self):
        b = self.board.copy()
        self.place_piece_on_board(b)
        return b

    def place_piece_on_board(self, board):
        piece = self.get_current_shape()

        for x in range(len(piece)):
            for y in range(self.pos_y, self.pos_y + len(piece[0])):
                if y < 0:
                    continue
                if piece[y - self.pos_y][x] == 0:
                    continue
                board[y] |= (0b1000000000 >> (self.pos_x + x))

    def get_current_shape(self):
        return self.TETROMINOS[self.bag[self.current_piece] * 4 + self.rotation]

    def get_piece_shape(self, piece):
        return self.TETROMINOS[piece * 4]

    def get_current_piece(self):
        return self.bag[self.current_piece]

    def is_game_over(self):
        return self.game_over

