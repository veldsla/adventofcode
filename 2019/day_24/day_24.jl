const DIRS = [CartesianIndex(0,1), CartesianIndex(1, 0), CartesianIndex(0,-1), CartesianIndex(-1,0)]

function inrange(c, dims)
	c[1]> 0 && c[2] >0 && c[1] <= dims[1] && c[2] <= dims[2]
end


function countaround(m, coord)
	dim = size(m)
	count = 0
	for d in DIRS
		n = coord + d
		if inrange(n, dim) && m[n] == '#'
			count += 1
		end
	end
	count
end

function life(m, c)
	n = countaround(m,c)
	if m[c] == '#' && n != 1
		'.'
	elseif m[c] == '.' && (n == 1 || n == 2)
		'#'
	else
		m[c]
	end
end

function live(m)
	 map(c->life(m,c), CartesianIndices(m))
end

function biodiversity(m)
	count = 0
	s = Set()
	n = live(m)
	while !(n in s)
		push!(s, n)
		n = live(n)
	end
	points = 2 .^ permutedims(reshape(0:24,(5,5)))
	sum(points[findall(n .== '#')])
end

#p2 stuff
function countfoldedaround(s, level, coord)
	m = s[level]
	count = 0
	for d in DIRS
		x, y = Tuple(coord + d)
		count += if x == 0
			o = s[level - 1]
			o[2,3] == '#' ? 1 : 0
		elseif y == 0
			o = s[level - 1]
			o[3,2] == '#' ? 1 : 0
		elseif x == 6
			o = s[level - 1]
			o[4,3] == '#' ? 1 : 0
		elseif y == 6
			o = s[level - 1]
			o[3,4] == '#' ? 1 : 0
		elseif x == 3 && y == 3
			o = s[level + 1]
			if d == CartesianIndex(0,1)
				sum(o[:,1] .== '#')
			elseif d == CartesianIndex(1,0)
				sum(o[1,:] .== '#')
			elseif d == CartesianIndex(-1,0)
				sum(o[5,:] .== '#')
			elseif d == CartesianIndex(0,-1)
				sum(o[:,5] .== '#')
			end 
		else 
			m[x,y] == '#' ? 1 : 0
		end
	end
	count
end

#count all levels, create new level neighbors and count those too
function countfolded!(s)
	counts = Dict{Int64,Array{Int64,2}}()

	levels = collect(keys(s))
	addedouter = minimum(levels) - 1
	addedinner = maximum(levels) + 1
	s[addedouter] = fill('.', 5, 5)
	s[addedinner] = fill('.', 5, 5)
	for l in levels
		lc = get!(counts, l, zeros(Int64,5,5))

		for c in CartesianIndices(s[l])
			if c == CartesianIndex(3,3)
				continue
			end
			lc[c] = countfoldedaround(s,l,c)
		end
	end
	#in the added levels bugs can only spawn on the inner/outer rings
	lc = get!(counts, addedouter, zeros(Int64, 5, 5))
	for coord in map(d->d+CartesianIndex(3,3), DIRS)
		lc[coord] = countfoldedaround(s, addedouter, coord)
	end

	lc = get!(counts, addedinner, zeros(Int64, 5, 5))
	# the border of the inner matrix get the 1 or zero depending on
	# the neighboring cell
	c8 = s[addedinner-1][3,2] == '#' ? 1 : 0;
	c12 = s[addedinner-1][2,3] == '#' ? 1 : 0;
	c14 = s[addedinner-1][4,3] == '#' ? 1 : 0;
	c18 = s[addedinner-1][3,4] == '#' ? 1 : 0;
	lc[:,1] .+= c8
	lc[1,:] .+= c12
	lc[5,:] .+= c14
	lc[:,5] .+= c18

	counts
end

function bugslife((b, c))
	if b == '#' && c != 1
		'.'
	elseif b == '.' && (c == 1 || c == 2)
		'#'
	else
		b
	end
end

function livefolded!(s, counts)
	for l in collect(keys(s))
		m = s[l]
		c = counts[l]
		s[l] = map(bugslife, zip(m,c))
	end
	# cleanup for unoccupied levels?
	# if not we will grow on each iteration
end

function lifefolded(m, n=100)
	s = Dict{Int64,Array{Char,2}}(0=>m)
	for i in 1:n
		counts = countfolded!(s)
		livefolded!(s, counts)
	end

	nbugs = 0
	for v in values(s)
		nbugs += sum(v.=='#')
	end
	nbugs
end

#part 1
eris = permutedims(map(x->x[1], hcat((split.(readlines("input.txt"),""))...)))
bd = biodiversity(eris)
println("Day 24 part 1: Eris' biodiversity is $bd")

#part 2
t = permutedims(map(x->x[1], hcat((split.(readlines("test.txt"),""))...)))
@assert lifefolded(t, 10) == 99
nbugs = lifefolded(eris, 200)
println("Day 24 part 2: Eris' numbher of bugs after 200 minutes is $nbugs")

