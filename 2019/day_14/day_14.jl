struct Part
	name::String
	quantity
end

struct Reaction
	input::Vector{Part}
	output::Part
end

function parsereaction(s)
	function pp(p)
		(n, t) = split(p, " ")
		Part(t, parse(Int64, n))
	end
	
	rin, rout = map(l->pp.(split(l, ", ")), split(s, " => "))
	Reaction(rin, rout[1])
end

function calcore(r, fuel=1)
	want = Dict([("FUEL", fuel)])
	done = Dict(map(x->(x.output.name, false), r))

	queue = Set(["FUEL"])
	ore = 0

	while !all(values(done))
		product = pop!(queue)
		i = findfirst(p->p.output.name == product, r)
		react = r[i]
		n = want[react.output.name] ÷ react.output.quantity + (want[react.output.name] % react.output.quantity == 0 ? 0 : 1)
		#We need the inputs n times
		for input in react.input
			if input.name != "ORE"
				if haskey(want, input.name)
					want[input.name] += n * input.quantity
				else
					want[input.name] = n * input.quantity
				end
			else
				ore += n * input.quantity
			end
		end
		done[react.output.name] = true
		#queue the reactions that are wanted and not done
		#and have no inputs that arent counted yet in not done reactions
		#this only works if we use all reactions in the collection
		for out in filter(x->!done[x], collect(keys(want)))
			add = true
			for nr in filter(x->!done[x.output.name], r)
				if any(x->x.name == out, nr.input)
					add = false
				end
			end

			if add
				push!(queue, out)
			end
		end
	end
	ore
end

function canmake(r, ore)
	#do binsearch to get the answer
	#std lib has searchsortedfirst/last, but I can't get it to work
	size = ore
	base = 1;
	while size > 1
		half = size ÷ 2
		mid = base + half;
		y = calcore(reactions, mid)
		base = if y > ore
			base
		else
			mid
		end
		size -= half
	end
	base
end


@assert calcore(parsereaction.(readlines("test.txt"))) == 165
@assert calcore(parsereaction.(readlines("test2.txt"))) == 13312
@assert calcore(parsereaction.(readlines("test3.txt"))) == 180697
@assert calcore(parsereaction.(readlines("test4.txt"))) == 2210736

reactions = parsereaction.(readlines("input.txt"))
required = calcore(reactions)
println("Day 14 part 1: Required ore is $required")

numfuel = canmake(reactions, 1000000000000)
println("Day 14 part 2: 1000000000000 ore makes $numfuel fuel")





