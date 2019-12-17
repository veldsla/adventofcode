include("../day_09/Intcode.jl")
using .Intcode	

const DIRS = [CartesianIndex(0,1), CartesianIndex(1, 0), CartesianIndex(0,-1), CartesianIndex(-1,0)]

function getfield(c)
	run!(c)
	s = String(convert(Vector{UInt8}, collect(c.output)))
	permutedims(hcat(split.(filter(x->!isempty(x), split(s, "\n")),"")...))
end

function draw(m)
	for i in 1:size(m,1)
		println(join(m[i,:]))
	end
end

function inrange(c, dims)
	c[1]> 0 && c[2] >0 && c[1] <= dims[1] && c[2] <= dims[2]
end
	

function walktheline(m, draw=false)
	s = size(m)
	#do we always start up, or just me?
	@show coord = findfirst(x->x=="^", m)
	dir = findfirst(d->inrange(coord+d, s) && m[coord+d] == "#", DIRS)
	println("Starting $dir")

	seen = Set()
	intersections = []
	while !isnothing(dir)
		d = DIRS[dir]
		if inrange(coord+d, s) && m[coord + d] == "#"
			if draw
				m[coord+d] = "-"
				draw(m)
				m[coord+d] = "#"
				sleep(0.05)
			end

			push!(seen, coord)
			coord += d
			if coord in seen
				push!(intersections, coord)
			end
		else
			#turn
			dir = findfirst(t->!(coord+t in seen) && inrange(coord+t, s) && m[coord+t] == "#", DIRS)
		end
	end
	intersections
end

#prefer turning right
const moves = [["R"], ["R","R"], ["L"], ["error"], ["R"],["R","R"], ["L"]]
function turn(d1, d2)
	diff = d2 - d1 + 4
	moves[diff]
end

#like walk, but collect the path as a string
function getpath(m)
	s = size(m)
	# we start up (-1, 0) or dir 4
	coord = findfirst(x->x=="^", m)
	from = coord
	@show dir = findfirst(d->inrange(coord+d, s) && m[coord+d] == "#", DIRS)
	# turn to dir
	println("goiing $(DIRS[dir])")
	path = []
	@show append!(path,turn(4, dir))

	seen = Set()
	steps = 0
	while !isnothing(dir)
		d = DIRS[dir]
		if inrange(coord+d, s) && m[coord + d] == "#"
			push!(seen, coord)
			coord += d
			steps += 1
		else
			push!(path, string(steps))
			#turn
			newdir = findfirst(t->!(coord+t in seen) && inrange(coord+t, s) && m[coord+t] == "#", DIRS)
			if isnothing(newdir)
				break
			end
			append!(path, turn(dir, newdir))
			dir = newdir
			steps = 0
		end
	end
	path
end

function partition(v)
	s = join(v, ",")
	possible = Dict()
	for l in 20:-1:2
		for	ss in 1:length(v)-l
			m = findall(join(v[ss:ss+l], ","),s)
			if length(m) >= 2
				locs = get!(possible, s[m[1]], Set())
				for r in m
					push!(locs, r)
				end
			end
		end
	end
	possible
end

function pathfromparts(c, s)
	#combine 3 parts into the entire sequence
	parts = collect(keys(c))
	sort!(parts, by=x->length(x), rev=true)
end

function tryparts(p, map, s)
	v = fill(false, length(s))
	for pa in p
		for r in map[pa]
			if any(v[r])
				println( "overlap")
			end
			v[r] .= true
			println(join(v,""))
		end
	end
	if s[findall(v)] .== ","
		true
	else
		false
	end
end

function warnbots(c, main, A, B, C)
end


#convert to ascii decimals
cd = s -> map(i->convert(Int, s[i]), 1:length(s))




tf = """
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."""
test = permutedims(hcat(split.(filter(x->!isempty(x), split(tf, "\n")),"")...))
@assert sum(map(x->(x[1]-1)*(x[2]-1), walktheline(test))) == 76

tf2 = """
#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......"""
test2 = permutedims(hcat(split.(filter(x->!isempty(x), split(tf2, "\n")),"")...))
tp2 = getpath(test2)
tpp = partition(tp2)



c = Computer("input.txt")
field = getfield(copy(c))
draw(field)
isect = walktheline(field)
p1 = sum(map(x->(x[1]-1)*(x[2]-1),isect))
println("Day 17 part 1: $(length(isect)) intersections have alignment parameter sum $p1")



path = getpath(field)
parts = partition(path)
ps = collect(keys(parts))
sort!(ps, by=x->length(x)*length(parts[x]), rev=true)

for key in ps
	println(key,"\t",sort(collect(parts[key])))
end
# 11111111111111              11111111111111                                                                                    11111111111111
#               22222222222222                                 22222222222222                  2222222222222
# R,12,L,10,R,12,L,8,R,10,R,6,R,12,L,10,R,12,R,12,L,10,R,10,L,8,L,8,R,10,R,6,R,12,L,10,R,10,L,8,L,8,R,10,R,6,R,12,L,10,R,10,L,8,R,12,L,10,R,12,R,12,L,10,R,10,L,8
# R,12,L,10,R,12
# ,L,8,R,10,R,6
# ,R,12,L,10,R,10,L,8

main = "A,B,A,C,B,C,B,C,A,C\n"
A = "R,12,L,10,R,12\n"
B = "L,8,R,10,R,6\n"
C = "R,12,L,10,R,10,L,8\n"
feed = "n\n"

c = Computer("input.txt")
c.mem[1] = 2
task = @async run!(c)
for v in cd(main)
	put_input!(c, v)
end
for v in cd(A)
	put_input!(c, v)
end
for v in cd(B)
	put_input!(c, v)
end
for v in cd(C)
	put_input!(c, v)
end
for v in cd(feed)
	put_input!(c, v)
end

res = collect(c.output)
println("Day 17 part two: Space dust collected: $(res[end])")







