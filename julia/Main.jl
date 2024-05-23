include("Tetrises.jl")
include("ConsoleUIs.jl")

using .Tetrises
using .ConsoleUIs

tetris = Tetris()

while true
    draw(tetris)
    gravity(tetris)
    sleep(1/10)
end