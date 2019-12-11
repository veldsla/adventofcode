include("../day_09/Intcode.jl")
using .Intcode

#turn right order
const DIRS = [CartesianIndex(0,1), CartesianIndex(1, 0), CartesianIndex(0,-1), CartesianIndex(-1,0)]

function paint(c, init=0)
	painted = Dict()
	task = @async run!(c)

	coord = CartesianIndex(0, 0)
	dir = 1

	while !istaskdone(task)
		put_input!(c, get(painted, coord, init))

		t = take_output!(c)
		painted[coord] = t

		d = take_output!(c)
		if d == 0
			dir = dir == 1 ? 4 : dir - 1
		else
			dir = dir == 4 ? 1 : dir + 1
		end
		coord += DIRS[dir]
	end
	painted
end

robot = Computer("input.txt")
r1 = copy(robot)
painted = paint(r1)
println("Day 11 part 1: Number of painted panels = $(length(painted))")

painted = paint(robot, 1)
# get the dimensions
o = CartesianIndex(1,1)
coords = collect(keys(painted))
minc = minimum(coords)
dims = maximum(map(k->k - minc, coords))
m = fill("░", Tuple(dims + o))
for c in coords
	if painted[c] == 1
		m[c - minc + o] = "█"
	end
end
image = permutedims(m)
println("Day 8 part 2: Painted identifier below:")
for i in size(image,1):-1:1
	println(join(image[i,:]))
end











