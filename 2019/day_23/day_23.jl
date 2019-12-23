include("../day_09/Intcode.jl")
using .Intcode	

mutable struct Switch
	nodes::Vector{Computer}
	tasks
	nat
	natsent	
end

function Switch(s::String, n)
	nodes = []
	tasks = []
	for i in 0:n-1
		c = Computer(s)
		t = @async run!(c, [i,-1])
		push!(nodes, c)
		push!(tasks, t)
	end
	Switch(nodes, tasks, nothing, Dict())
end

function switch!(s::Switch, testrun=false)
	while true
		yield()
		if all(c->!isready(c.input)	 && !isready(c.output), s.nodes	)
			#idle network
			put_input!(s.nodes[1], -1)
			put_input!(s.nodes[1], s.nat[1])
			put_input!(s.nodes[1], s.nat[2])

			s.natsent[s.nat[2]] = get!(s.natsent, s.nat[2],0) + 1
			if s.natsent[s.nat[2]] == 2
				return s.nat[2]
			end

		end

		for c in s.nodes
			if has_output(c)
				addr = take_output!(c)
				x = take_output!(c)
				y = take_output!(c)
				if addr == 255
					if testrun
						return y
					end
					s.nat = (x,y)
					continue
				end

				if !isready(s.nodes[addr+1].input)
					put_input!(s.nodes[addr+1], -1)
				end
				put_input!(s.nodes[addr+1], x)
				put_input!(s.nodes[addr+1], y)
			end
		end
	end
end


s = Switch("input.txt", 50)
y255 = switch!(s, true)
println("Day 23 part 1: The Y value sent to addr 255 is $y255")

s2 = Switch("input.txt", 50)
naty = switch!(s2)
println("Day 23 part 2: The Y value sent by the NAT twice is $naty")


