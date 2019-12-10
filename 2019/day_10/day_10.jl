function max_visible(m, occ, x, y)
	sx = size(m,1)
	sy = size(m,2)
	@assert m[x,y] == "#"

	seen = Set([(x, y)])
	blocked = Set()
	for coord in occ
		#println("to = $coord")
		if Tuple(coord) in seen || Tuple(coord) in blocked
			continue
		end

		dx = coord[1] - x
		dy = coord[2] - y

		#simplify
		lcd = gcd(dx, dy)
		dx = dx ÷ lcd
		dy = dy ÷ lcd
		#println("dx=$dx, dy=$dy")

		# find all asteroids on the line x,y to this asteroid
		# count the closest visible mark remainder blocked

		# cannot collect uneven sized iterators and also cannot filter a 
		# zip iterator....way to go Julia
		rx = dsteps(x, dx, sx)
		ry = dsteps(y, dy, sy)
		points = zip(rx, ry)
		coords = collect(Iterators.take(points, min(length(rx), length(ry))))
		
		# get the asteroids coordinates from the grid
		filter!(c->m[c[1], c[2]] == "#", coords)

		if !isempty(coords)
			push!(seen, coords[1])
			for c in coords[2:end]
				push!(blocked, c)
			end
		end
	end
	
	length(seen) - 1
end

function dsteps(start, step, max)
	if step == 0
		repeat([start], max)
	else
		e = step > 0 ? max : 1
		start+step:step:e
	end
end

function zap(occ, from, nth)
	# sort on angle break ties on distance
	angle = map(x->atan(from[1] - x[1], from[2] - x[2]), occ)
	at = map(a -> a <= 0 ? 2pi-abs(a) : a , angle)
	dist = map(x->sqrt((from[1] - x[1])^2 + (from[2] - x[2])^2), occ)
	ord = sort(1:length(occ), by=x->(2pi-at[x], dist[x]))

	ats = at[ord]
	occs = occ[ord]
	ds = dist[ord]
	
	# zap full rounds
	count = 0
	while !isempty(occs)
		tozap = unique(x->ats[x], 1:length(occs))
		println("count = $count, willzap $(length(tozap))")
		if count + length(tozap) >= nth
			return occs[tozap[nth-count]]
		end
		deleteat!(ats, tozap)
		deleteat!(occs, tozap)
		deleteat!(ds, tozap)
		count += length(tozap)
	end
end

	


t1 = hcat(split.(readlines("test2.txt"), "")...)
tc = findall(t1 .== "#")
(max, i) = findmax(map(c -> max_visible(t1, tc, c[1], c[2]), tc))
@assert max == 41
@assert tc[i] == CartesianIndex(7, 4)

t1 = hcat(split.(readlines("test3.txt"), "")...)
tc = findall(t1 .== "#")
(max, i) = findmax(map(c -> max_visible(t1, tc, c[1], c[2]), tc))
@assert max == 210
@assert tc[i] == CartesianIndex(12, 14)
from = tc[i]
t1[tc[i]] = "X"
tc = findall(t1 .== "#")
@assert zap(tc, from, 200) == CartesianIndex(9, 3)
@assert zap(tc, from, 299) == CartesianIndex(12, 2)

a = hcat(split.(readlines("input.txt"), "")...)
# where are the asteroids
ac = findall(a .== "#")
(max, i) = findmax(map(c -> max_visible(a, ac, c[1], c[2]), ac))
println("Day 10 part 1: Maximum visible from ($(ac[i][1]-1), $(ac[i][2]-1)) is $max")

from = ac[i]
a[from] = "X"
ac = findall(a .== "#")
zap200 = zap(ac, from, 200)
println("Day 10 part 1: 200th asteroid to zap is at ($(zap200[1] -1),$(zap200[2] -1)) answer = $((zap200[1]-1) * 100 + zap200[2] -1)")



