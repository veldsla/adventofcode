module Intcode

mutable struct Computer
	mem::Array{Int64}
	pos
	input::Channel{Int64}
	output::Channel{Int64}
end

function Computer(s::String)
	Computer(parse.(Int64, split(readline("input.txt"), ',')))
end

function Computer(v::Vector{Int64})
	Computer(v, 1, Channel{Int64}(Inf), Channel{Int64}(Inf))
end

Base.copy(c::Computer) = Computer(copy(c.mem))

function decode(c::Computer)
	op = c.mem[c.pos]
	opcode = op % 10
	p1 = op ÷ 100 % 10
	p2 = op ÷ 1000 % 10
	p3 = op ÷ 10000 % 10
	(opcode, p1, p2, p3)
end

"Get the value from the memory vector v[v[pos]] or v[pos] depending on mode" 
function get(c, pos, mode)
	if mode == 0
		c.mem[c.mem[pos]+1]
	else
		c.mem[pos]
	end
end

"Write is alway in position mode, write to the location stored at pos"
function set!(c, pos, value)
	c.mem[c.mem[pos]+1] = value
end

# the opcode implementations directly modify the memory vector
# pos is the location of the opcode, parameters start at pos+1
# p1 .. p3 indicate the parameter mode (position/immediate), not all
# are required, but all are provided.
function add!(c, pos, p1, p2, p3)
	set!(c, pos+3, get(c, pos+1, p1) + get(c, pos+2, p2))
	4
end

function mul!(c, pos, p1, p2, p3)
	set!(c, pos+3, get(c, pos+1, p1) * get(c, pos+2, p2))
	4
end

function save!(c, pos, p1, p2, p3)
	set!(c, pos+1, take!(c.input))
	2
end

function print(c, pos, p1, p2, p3)
	out = get(c, pos+1, p1)
	put!(c.output, out)
	2
end

function jift(c, pos, p1, p2, p3)
	pos = get(c, pos+1, p1) != 0 ? get(c, pos+2, p2) - pos + 1 : 3
	pos
end

function jiff(c, pos, p1, p2, p3)
	pos = get(c, pos+1, p1) != 0 ? 3 : get(c, pos+2, p2) - pos + 1 # 1-based...
	pos
end

function lessthan!(c, pos, p1, p2, p3)
	if get(c, pos+1, p1) < get(c, pos+2, p2)
		set!(c, pos+3, 1)
	else
		set!(c, pos+3, 0)
	end
	4
end

function equals!(c, pos, p1, p2, p3)
	if get(c, pos+1, p1) == get(c, pos+2, p2)
		set!(c, pos+3, 1)
	else
		set!(c, pos+3, 0)
	end
	4
end

function put_input!(c::Computer, value)
	put!(c.input, value)
end

function take_output!(c::Computer)
	take!(c.output)
end

function has_output(c::Computer)
	isready(c.output)
end

const opcodes = Dict(1=>add!, 2=>mul!, 3=>save!, 4=>print, 5=>jift, 6=>jiff, 7=> lessthan!, 8=>equals!)

"Run the computer starting at the set memory position (starts at 1)"
function run!(comp::Computer, input=missing)
	#println("Running computer from position $(comp.pos) using input $input")
	if !ismissing(input)
		for v in input
			put!(comp.input, v)
		end
	end
	(opcode, p1, p2, p3) = decode(comp)
	while haskey(opcodes, opcode)
		comp.pos += opcodes[opcode](comp, comp.pos, p1, p2, p3);
		(opcode, p1, p2, p3) = decode(comp)
	end
	if isready(comp.output)
		fetch(comp.output)
	end
end

export Computer, run!, put_input!, take_output!, has_output

end
