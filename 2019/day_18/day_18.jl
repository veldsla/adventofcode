const DIRS = [CartesianIndex(0,1), CartesianIndex(1, 0), CartesianIndex(0,-1), CartesianIndex(-1,0)]

function draw(m)
	colors = Dict('#'=>"█", '.'=>"░")
	for i in 1:size(m,1)
		println(join(map(c->get(colors,c,c), m[i,:])))
	end
end

function getkeys(m, found=false)
	keys = Dict{Char, Bool}()
	for c in m
		if 'a' <= c <='z'
			keys[c] = found
		end
	end
	keys
end

function inrange(c, dims)
	c[1]> 0 && c[2] >0 && c[1] <= dims[1] && c[2] <= dims[2]
end

function solve(maze, start, foundkeys, keycache::Dict{String, Int64}, steps)
	if all(values(foundkeys))
		if !haskey(keycache, "finish") || keycache["finish"] > steps
			keycache["finish"] = steps
		end
		return steps, start
	end

	#do a bfs and find all available keys from start
	seen = Set{CartesianIndex{2}}()
	queue = [[start]]
	next = []
	found = Dict{CartesianIndex{2}, Int64}()
	while !isempty(queue) && steps < get(keycache, "finish", 999999999)
		list = popfirst!(queue)
		steps += 1

		empty!(next)
		for coord in list
			push!(seen, coord)
			for d in DIRS
				n = coord + d
				mtype = maze[n]
				if inrange(n, size(maze)) && !(n in seen)
					if mtype == '.' || mtype == '@'
						push!(next, n)
					elseif 'A' <= mtype <= 'Z'
						if get(foundkeys, mtype+32, false)
							#door open
							push!(next, n)
						end
					elseif 'a' <= mtype <= 'z'
						if !foundkeys[mtype]
							found[n] = steps
						end
						#continue walking?
						push!(next, n)
					end
				end
			end
		end
		if !isempty(next)
			push!(queue, next)
		end
	end

	donesteps = []
	keysid = join(filter(x->foundkeys[x], sort(collect(keys(foundkeys)))))

	for (k, s) in found

		#stop traversal if already finished in less steps
		if haskey(keycache,"finish") && keycache["finish"] <= s
			continue
		end

		#stop traversal if same collection was found at same coordinate in less steps
		kkeysid = string(keysid, maze[k])
		if !haskey(keycache, kkeysid) || keycache[kkeysid] > s
			#mark key as found and continue searching
			nk = copy(foundkeys)
			nk[maze[k]] = true
			keycache[kkeysid] = s
			done, lc = solve(maze, k, nk, keycache, s)
			if !isnothing(done)
				push!(donesteps, (done, lc))
			end
		end
	end

	if !isempty(donesteps)
		(v, i) = findmin(map(x->x[1], donesteps))
		donesteps[i]
	else
		nothing, nothing, false
	end
end

function splitmaze(m)
	m[40:42,40:42] = ['@','#','@','#','#','#','@','#','@']
	(m[1:41,1:41], m[41:81,1:41], m[1:41,41:81], m[41:81, 41:81])
end

#testmaze = permutedims(map(x->x[1], hcat(split.(readlines("test.txt"),"")...)))
#draw(testmaze)
#testkeys = getkeys(testmaze)
#teststart = findfirst(testmaze.=='@')
#@time testsol = solve(testmaze, teststart, testkeys, Dict{String, Int64}(),0)
#@assert testsol[1] == 136


fullmaze = permutedims(map(x->x[1], hcat(split.(readlines("input.txt"),"")...)))
draw(fullmaze)

# There appear to be 4 quandrants, which hopefully can be solved consequetively
# Because it sounds a lot like the travelling salesman problem, slightly restricted by
# the availbale keys. Tough luck. Brute force and slightly naive it is....
mstart = findfirst(fullmaze.=='@')
mkeys = getkeys(fullmaze)
@time minsteps = solve(fullmaze, mstart, mkeys, Dict{String, Int64}(),0)
println("Day 18 part 1: Found all keys in $(minsteps[1]) steps")

#lots of possible optimizations. convert to adjacency list, precalculating routes?

#part 2
#let's guess the shortest paths in the split mazes are good enough in picking up the correct keys
(maze1,maze2,maze3,maze4) = splitmaze(fullmaze)
m1 = solve(maze1, findfirst(maze1.=='@'), merge(getkeys(fullmaze, true),getkeys(maze1)), Dict{String, Int64}(),0)
m2 = solve(maze2, findfirst(maze2.=='@'), merge(getkeys(fullmaze, true),getkeys(maze2)), Dict{String, Int64}(),0)
m3 = solve(maze3, findfirst(maze3.=='@'), merge(getkeys(fullmaze, true),getkeys(maze3)), Dict{String, Int64}(),0)
m4 = solve(maze4, findfirst(maze4.=='@'), merge(getkeys(fullmaze, true),getkeys(maze4)), Dict{String, Int64}(),0)

println("Day 18 part 2: Hopefully the lowest number of steps to collect all keys is $(m1[1]+m2[1]+m3[1]+m4[1])")






