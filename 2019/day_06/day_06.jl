function load_orbit_graph(file)
	orbits = Dict{String, Vector{String}}()
	for (body, object) in split.(readlines(file),')')
		o = get!(orbits, body, [])
		push!(o, object)
		# insert entry for object so all have an entry in orbits
		get!(orbits, object, [])
	end
	orbits
end

function count_around(orbits, body, orbcounts=Dict())
	around = orbits[body]
	sum = length(around)
	for obj in around
		num = get!(orbcounts, obj, count_around(orbits, obj, orbcounts))
		sum += num
	end
	sum
end

function find_up(orbits, from, dest, dist=0)
	dist += 1
	if length(orbits[from]) == 0
		return missing
	end

	if dest in orbits[from]
		return dist-1
	end

	for n in orbits[from]
		d = find_up(orbits, n, dest, dist)
		if !ismissing(d)
			return d
		end
	end
	missing
end

function dist_to_both(orbits, from, t1, t2)
	find_up(orbits, from, t1) + find_up(orbits, from, t2)
end

test = load_orbit_graph("test.txt")
@assert sum(map(k -> count_around(test, k), collect(keys(test)))) == 42

test2 = load_orbit_graph("test2.txt")
@assert minimum(skipmissing(map(from->dist_to_both(test2, from, "SAN", "YOU"), collect(keys(test))))) == 4


o = load_orbit_graph("input.txt")
num_orbits = sum(map(k -> count_around(o, k), collect(keys(o))))
println("Day 6 part 1: Number of orbits is $num_orbits")

# let's waste some cycles by checking if both targets can be found from any starting point
# summarize to minimum to get the answer
commonbranchpointdist = minimum(skipmissing(map(from->dist_to_both(o, from, "SAN", "YOU"), collect(keys(o)))))
println("Day 6 part 2: Minimum distance between SAN and YOU is $commonbranchpointdist")

