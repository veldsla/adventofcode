module Intcode

mutable struct Computer
	mem::Array{Int64}
	pos
end

function Computer(s::String)
	Computer(parse.(Int64, split(readline("input.txt"), ',')), 1)
end

function decode(c::Computer)
	op = c.mem[c.pos]
	opcode = op % 10
	p1 = op ÷ 100 % 10
	p2 = op ÷ 1000 % 10
	p3 = op ÷ 10000 % 10
	(opcode, p1, p2, p3)
end

"Get the value from the memory vector v[v[pos]] or v[pos] depending on mode" 
function get(v, pos, mode)
	if mode == 0
		v[v[pos]+1]
	else
		v[pos]
	end
end

"Write is alway in position mode, write to the location stored at pos"
function set!(v, pos, value)
	v[v[pos]+1] = value
end

# the opcode implementations directly modify the memory vector
# pos is the location of the opcode, parameters start at pos+1
# p1 .. p3 indicate the parameter mode (position/immediate), not all
# are required, but all are provided.
# input is the computer input (currently only relevant in save!)
function add!(v, pos, p1, p2, p3, input = missing)
	set!(v, pos+3, get(v, pos+1, p1) + get(v, pos+2, p2))
	4
end

function mul!(v, pos, p1, p2, p3, input = missing)
	set!(v, pos+3, get(v, pos+1, p1) * get(v, pos+2, p2))
	4
end

function save!(v, pos, p1, p2, p3, input)
	set!(v, pos+1, input)
	2
end

function print(v, pos, p1, p2, p3, input = missing)
	println(get(v, pos+1, p1))
	2
end

function jift(v, pos, p1, p2, p3, input = missing)
	if get(v, pos+1, p1) != 0
		get(v, pos+2, p2) - pos + 1 # 1-based...
	else
		3
	end
end

function jiff(v, pos, p1, p2, p3, input = missing)
	if get(v, pos+1, p1) != 0
		3
	else
		get(v, pos+2, p2) - pos + 1 # 1-based...
	end
end

function lessthan!(v, pos, p1, p2, p3, input = missing)
	if get(v, pos+1, p1) < get(v, pos+2, p2)
		set!(v, pos+3, 1)
	else
		set!(v, pos+3, 0)
	end
	4
end

function equals!(v, pos, p1, p2, p3, input = missing)
	if get(v, pos+1, p1) == get(v, pos+2, p2)
		set!(v, pos+3, 1)
	else
		set!(v, pos+3, 0)
	end
	4
end


const opcodes = Dict(1=>add!, 2=>mul!, 3=>save!, 4=>print, 5=>jift, 6=>jiff, 7=> lessthan!, 8=> equals!)

"Run the computer starting at the set memory position (starts at 1)"
function run!(comp::Computer, input=missing)
	println("Running computer from position $(comp.pos) using input $input")
	(opcode, p1, p2, p3) = decode(comp)
	while haskey(opcodes, opcode)
		comp.pos += opcodes[opcode](comp.mem, comp.pos, p1, p2, p3, input)
		(opcode, p1, p2, p3) = decode(comp)
	end
end

export Computer, run!

end
