include("Intcode.jl")
using .Intcode

c = Computer("input.txt")
println("Day 5 part 1: Run output:")
run!(c, 1)

println()

c = Computer("input.txt")
println("Day 5 part 2: Run output:")
run!(c, 5)


