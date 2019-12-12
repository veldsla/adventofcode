function parsemoon(s::String)
	re = r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>"
	m = match(re, s)
	parse.(Int, m.captures)
end

function update!(m, v)
	n = size(m,1)
	for x in 1:n-1, y in x+1:n

		dv = sign.(m[x,:] .- m[y,:])
		v[x,:] .-= dv
		v[y,:] .+= dv
	end
	m .+= v
end

function energy(m, v)
	sum(sum(abs.(m),dims=2) .* sum(abs.(v),dims=2))
end

function perioddim(md, vd)
	c = Set{Vector{Int64}}()
	n = size(md,1)
	step = 0
	while true
		step += 1
		for x in 1:n-1, y in x+1:n
			dv = sign(md[x] - md[y])
			vd[x] -= dv
			vd[y] += dv
		end
		md .+= vd
		key = vcat(md, vd)
		if key in c
			return step -1
		else
			push!(c, key)
		end
	end
end

function getperiod(v)
	period = prod(v)
	d = gcd(period .÷ v)
	period ÷= d
end


tm = permutedims(reshape(vcat(parsemoon.(readlines("test.txt"))...), (3,4)))
tv = zeros(Int64,size(tm))
for i in 1:10
	update!(tm, tv)
end
@assert energy(tm, tv) == 179 

tm = permutedims(reshape(vcat(parsemoon.(readlines("test2.txt"))...), (3,4)))
tv = zeros(Int64,size(tm))
for i in 1:100
	update!(tm, tv)
end
@assert energy(tm, tv) == 1940 

tm = permutedims(reshape(vcat(parsemoon.(readlines("test2.txt"))...), (3,4)))
tv = zeros(Int64,size(tm))
tperiods = map(i->perioddim(tm[:,i], tv[:,i]), 1:3)
@assert getperiod(tperiods) == 4686774924

### Day 12
moons = permutedims(reshape(vcat(parsemoon.(readlines("input.txt"))...), (3,4)))
velocities = zeros(Int64,size(moons))
for i in 1:1000
	update!(moons, velocities)
end
e = energy(moons, velocities)
println("Day 12 part 1: Total energy in system after 1000 steps = $(e)")


#x, y, z behave independently, find period in each
moons = permutedims(reshape(vcat(parsemoon.(readlines("input.txt"))...), (3,4)))
velocities = zeros(Int64,size(moons))
periods = map(i->perioddim(moons[:,i], velocities[:,i]), 1:3)
period = getperiod(periods)
println("Day 12 part 2: Universe repeats after $(period) steps")





