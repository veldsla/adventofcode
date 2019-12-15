include("../day_09/Intcode.jl")
using .Intcode

const DIRS = [CartesianIndex(0,1), CartesianIndex(0, -1), CartesianIndex(-1,0), CartesianIndex(1,0)]
reverse = d->[2,1,4,3][d]

function draw(b)
	o = CartesianIndex(1,1)
	coords = collect(keys(b))
	minc = minimum(coords)
	dims = maximum(coords) - minimum(coords)

	m = fill(1, Tuple(dims + o))
	for c in coords
		m[c - minc + o] = b[c][1]
	end
	image = permutedims(m)
	colors = ["█", " ", "o","░", "X"]
	for i in 1:size(image,1)
		println(join(colors[image[i,:] .+ 1]))
	end
end

function find_ox(c)
	task = @async run!(c)

	coord = CartesianIndex(0,0)
	lastmoves = []
	dist = 0
	mintoox = typemax(Int64)
	coordox = (0,0)


	# store type and distance
	maze = Dict(coord => (4, 0))

	function try_dir(d)
		target = coord + DIRS[d]
		if haskey(maze, target)
			maze[target]
		else
			put_input!(c, d)
			res = take_output!(c)
			#and walk back....coward
			if res != 0
				put_input!(c, reverse(d))
				take_output!(c)
			end
			(res, nothing)
		end
	end

	while !istaskdone(task)
		#get a direction
		avail = []
		for dir in 1:4
			target = coord + DIRS[dir]
			ttype, tdist = try_dir(dir)

			if ttype == 0
				#wall
				maze[target] = (0, dist + 1)
				continue
			end

			if ttype == 3
				#backtracked
				continue
			end

			if ttype == 2
				#target found
				maze[target] = (2, dist + 1)
				if dist + 1 < mintoox
					mintoox = dist + 1
					coordox = target
				end
			end

			if isnothing(tdist) || (!isnothing(tdist) && tdist > dist)
				# new or better route
				push!(avail, dir)
			end
		end
		
		if isempty(avail) && isempty(lastmoves)
			maze[coordox] = (2, mintoox)
			return mintoox, coordox, maze
		end

		if isempty(avail)
			#dead end, place marker and backup
			maze[coord] = (3, dist)
			lastmove = pop!(lastmoves)
			put_input!(c, reverse(lastmove))
			res = take_output!(c)

			coord -= DIRS[lastmove]
			dist -= 1
		else
			push!(lastmoves, avail[1])
			coord += DIRS[avail[1]]
			dist += 1

			put_input!(c, avail[1])
			p = take_output!(c)
			maze[coord] = (p, dist)
		end
	end
end

function diffuse(maze, from)
	#the diffuse time is the longest shortest route to any coordinate
	seen = Set()
	queue = [[from]]
	dist = 0
	while !isempty(queue)
		list = popfirst!(queue)
		dist += 1

		next = []
		for c in list
			push!(seen, c)

			for d in 1:4
				to = c + DIRS[d]
				if maze[to][1] != 0
					if !(to in seen)
						push!(next, to)
					end
				end
			end
		end
		if !isempty(next)
			push!(queue, next)
		end
	end
	dist - 1
end

c = Computer("input.txt")
dist, oxcoord, maze = find_ox(c)
println("Day 15 part 1: Distance to oxygen system is $dist steps")
draw(maze)

dtime = diffuse(maze, oxcoord)
println("Day 15 part 2: Time to oxygenize is $dtime minutes")




