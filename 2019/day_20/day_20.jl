const DIRS = [CartesianIndex(0,1), CartesianIndex(1, 0), CartesianIndex(0,-1), CartesianIndex(-1,0)]

function draw(m)
	colors = Dict('#'=>"█", '.'=>"░")
	for i in 1:size(m,1)
		println(join(map(c->get(colors,c,c), m[i,:])))
	end
end

function getportals(m)
	#find the letters on the outer edge
	rows, cols = size(m)
	portals = Dict()
	for l in findall(x->'A'<=x<='Z', m[1,:])
		c = get!(portals, join(m[1:2,l]), [])
		push!(c, CartesianIndex(3,l))
	end
	for l in findall(x->'A'<=x<='Z', m[rows,:])
		c = get!(portals, join(m[rows-1:rows,l]), [])
		push!(c, CartesianIndex(rows-2,l))
	end
	for l in findall(x->'A'<=x<='Z', m[:,1])
		c = get!(portals, join(m[l,1:2]), [])
		push!(c, CartesianIndex(l,3))
	end
	for l in findall(x->'A'<=x<='Z', m[:,cols])
		c = get!(portals, join(m[l,cols-1:cols]), [])
		push!(c, CartesianIndex(l,cols-2))
	end


	#and the inner
	tl = minimum(findall(x->'A'<=x<='Z',m[3:end-3,3:end-3])) + CartesianIndex(2,2)
	br = maximum(findall(x->'A'<=x<='Z',m[3:end-3,3:end-3])) + CartesianIndex(2,2)
	subm = m[tl[1]:br[1], tl[2]:br[2]]
	#draw(subm)
	rows, cols = size(subm)

	for l in findall(x->'A'<=x<='Z', subm[1,:])
		c = get!(portals, join(subm[1:2,l]), [])
		push!(c, CartesianIndex(-1,l-1) + tl)
	end
	for l in findall(x->'A'<=x<='Z', subm[rows,:])
		c = get!(portals, join(subm[rows-1:rows,l]), [])
		push!(c, CartesianIndex(rows,l-1)+tl)
	end
	for l in findall(x->'A'<=x<='Z', subm[:,1])
		c = get!(portals, join(subm[l,1:2]), [])
		push!(c, CartesianIndex(l-1,-1)+tl)
	end
	for l in findall(x->'A'<=x<='Z', subm[:,cols])
		c = get!(portals, join(subm[l,cols-1:cols]), [])
		push!(c, CartesianIndex(l-1,cols)+tl)
	end

	portals
	links = Dict()
	for l in values(portals)
		if length(l) == 2
			links[l[1]] = l[2]
			links[l[2]] = l[1]
		end
	end
	portals, links
end

function solve(maze, start, finish, portals)
	steps = 0
	seen = Set()
	queue = [[start]]
	while !isempty(queue)
		list = popfirst!(queue)
		steps += 1
		next = []
		for coord in list
			push!(seen, coord)
			#try walking
			for d in DIRS
				n = coord + d
				if n == finish
					return steps
				end
				if maze[n] == '.' && !(n in seen)
					push!(next, n)
				end
			end
			#or teleporting
			if haskey(portals, coord) && !(portals[coord] in seen)
				push!(next, portals[coord])
			end
		end
		if !isempty(next)
			push!(queue, next)
		end
	end
end

function solveall(maze, start, to)
	#reverse name coord map
	
	targets = Dict()
	for (n,c) in to
		targets[c] = n
	end

	result = []
	steps = 0
	seen = Set()
	queue = [[start]]
	while !isempty(queue)
		list = popfirst!(queue)
		steps += 1
		next = []
		for coord in list
			push!(seen, coord)
			if haskey(targets, coord) && coord != start
				push!(result, (targets[coord], steps-1))
			end

			#try walking
			for d in DIRS
				n = coord + d
				if maze[n] == '.' && !(n in seen)
					push!(next, n)
				end
			end
		end
		if !isempty(next)
			push!(queue, next)
		end
	end
	result
end

function graphportals(maze, portals)
	# connect portals for part2
	# rename according to inner or outer edge
	# and nest level to create vertex ids
	namedportals = Dict()
	for (name, coords) in portals
		if name == "ZZ"
			namedportals[(name,:outer,1)] = coords[1]
		elseif name != "AA"
			namedportals[(name,:outer,1)] = coords[1]
			namedportals[(name,:inner,1)] = coords[2]
		end
	end
	namedportals

	#find all routes to other portals
	routes = Dict()
	routes[("AA", :outer,1)] = solveall(maze, portals["AA"][1], namedportals)
	
	for (name, coord) in namedportals
		if name[1] != "ZZ"
			routes[name] = solveall(maze, coord, namedportals)
		end
	end

	routes
end

# rename vertices to level
# remove entry exit if level > 1
function addlevel!(gr, level)
	@assert level > 1
	for k in filter(x->x[3]+1 == level, collect(keys(gr)))
		if k[1] == "AA"
			continue
		end
		k
		nn = (k[1],k[2], k[3]+1)
		#copy adj list, but exclude ZZ
		gr[nn] = map(x->((x[1][1],x[1][2], x[1][3]+1),x[2]), filter(x->x[1][1] != "ZZ", gr[k]))
		gr[nn]

		# add to previous outer edge adj list
		if k[2] == :inner
			push!(gr[k], ((k[1], :outer, level),1))
		end

		#and link back outer ring to level up
		if nn[2] == :outer && nn[1] != "AA"
			push!(gr[nn], ((nn[1], :inner, level-1), 1))
		end
	end

	gr
end

function solvepart2(maze, portals)
	level = 1
	graph = graphportals(maze, portals)
	
	while true
		##append another inside layer
		@show level += 1
		addlevel!(graph, level)
		d = pathdijk(graph)
		if !isnothing(d)
			return d
		end
	end
end

function pathdijk(a)
	#compute shortest path from AA to ZZ if possible
	dists = Dict(("AA", :outer, 1)=>0)
	queue = [(("AA", :outer, 1),0)]
	dist = 0
	while !isempty(queue)
		n, d = popfirst!(queue)
		for (nn, nd) in a[n]
			if haskey(dists, nn) && dists[nn] <= d+nd
				continue
			end
			dists[nn] = d+nd
			#don't queue exit
			if nn[1] != "ZZ"
				push!(queue, (nn, d+nd))
			end
		end
	end
	get(dists, ("ZZ", :outer, 1), nothing)
end

function dot(gr)
	f = open("gr.dot","w")
	println(f,"digraph graphname {")
	for (n,l) in gr
		for v in l
			println(f,"$(n[1])$(n[2])$(n[3]) -> $(v[1][1])$(v[1][2])$(v[1][3]);")
		end
	end
	println(f,"}")
	close(f)
end


fullmaze = permutedims(map(x->x[1], hcat(split.(readlines("input.txt"),"")...)))
pbyname, plinks = getportals(fullmaze)
draw(fullmaze)

steps = solve(fullmaze, pbyname["AA"][1], pbyname["ZZ"][1], plinks)
println("Day 20 part 1: It takes $steps steps to go from AA to ZZ")

p2 = solvepart2(fullmaze, pbyname)
println("Day 20 part 2: It takes $p2 steps to go from AA to ZZ in recursive mode")





