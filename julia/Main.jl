include("Tetrises.jl")
include("ConsoleUIs.jl")

using .Tetrises
using .ConsoleUIs

tetris = Tetris()
draw(tetris)