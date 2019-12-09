include("Intcode.jl")
using .Intcode

#using Int64 appears to be enough, Int128, BigInt also work fine in the
#Intcode computer

p1 = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
t1 = Computer(copy(p1))
run!(t1)
@assert collect(t1.output) == p1
@assert run!(Computer([104,1125899906842624,99])) == 1125899906842624

c = Computer("input.txt")

boost = run!(copy(c), [1])
println("Day 9 part 1: Boost keycode is $boost")

coords = run!(c, [2])
println("Day 9 part 2: Distress signal coordinates are $coords")

