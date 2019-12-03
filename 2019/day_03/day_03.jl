using DelimitedFiles

function parse_path(s::String)
	(s[1], parse(Int64, s[2:end]))
end

# store coordinates as x, y
# DIRECTION has the Δ for each direction
const DIRECTION = Dict(
	'U' => [0,1],
	'D' => [0,-1],
	'R' => [1,0],
	'L' => [-1,0],
)

function makepath(p)
	coord = [0, 0]
	steps = 0
	path = Dict{Array, Int64}()
	for (dir, len) in parse_path.(p)
		for i in 1:len
			steps += 1
			coord += DIRECTION[dir]
			if !haskey(path, coord)
				path[coord] = steps
			end
		end
	end
	path
end

dist_to_origin = c -> sum(abs.(c))

paths = readdlm("input.txt", ',', String)

p1 = makepath(paths[1,:])
p2 = makepath(paths[2,:])

crossings = intersect(keys(p1), keys(p2))
closest = minimum(dist_to_origin.(crossings))
println("Day 3 part 1: Closest crossing at $closest")

minsteps = minimum(collect(map(k->p1[k]+p2[k], collect(crossings))))
println("Day 3 part 2: Minimum steps to crossing is $minsteps")
