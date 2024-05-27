include("Tetrises.jl")
include("ConsoleUIs.jl")

using .Tetrises
using .ConsoleUIs

tetris = Tetris()

function _get_char()
    ret = ccall(:jl_tty_set_mode, Int32, (Ptr{Cvoid},Int32), stdin.handle, true)
    ret == 0 || error("unable to switch to raw mode")
    c = read(stdin, Char)
    ccall(:jl_tty_set_mode, Int32, (Ptr{Cvoid},Int32), stdin.handle, false)
    c
end

function input_loop()

    while (true)
        c = _get_char()
        (c == 'q') && exit()
        (c == 'd') && move_left(tetris)
        (c == 'g') && move_right(tetris)
        (c == 'z') && hold(tetris)
        (Int(c) == 68) && rotate_left(tetris)
        (Int(c) == 67) && rotate_right(tetris)
        (Int(c) == 65) && hard_drop(tetris)
        (Int(c) == 66) && soft_drop(tetris)
    end
end

function draw_loop()
    
    print("\e[2J")

    while (true)
        print("\e[30A")
        draw(tetris)
        println("  " ^ 22)
        println("  " ^ 22)
        println("  " ^ 22)
        println("  " ^ 22)
        sleep(1/10)
    end
end

Threads.@spawn input_loop()
Threads.@spawn draw_loop()

while true

    gravity(tetris)
    if is_game_over(tetris)
        exit()
    end
    sleep(1/10)
end

