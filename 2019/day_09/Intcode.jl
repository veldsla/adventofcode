module Intcode

mutable struct Computer
	mem::Array{Int64}
	pos
	base
	input::Channel{Int64}
	output::Channel{Int64}
end

@enum Mode Position=0 Immediate=1 Relative=2

function Computer(s::String)
	Computer(parse.(Int64, split(readline(s), ',')))
end

function Computer(v::Vector{Int64})
	Computer(Array{Int64}(v))
end

function Computer(v::Vector{Int64})
	Computer(v, 1, 0, Channel{Int64}(Inf), Channel{Int64}(Inf))
end

Base.copy(c::Computer) = Computer(copy(c.mem))

function decode(c::Computer)
	op = c.mem[c.pos]
	opcode = op % 100
	p1 = op ÷ 100 % 10
	p2 = op ÷ 1000 % 10
	p3 = op ÷ 10000 % 10
	(opcode, Mode(p1), Mode(p2), Mode(p3))
end

#Get the value from the memory vector v[v[pos]+1] or v[pos] depending on mode
#pos is the 1 based location in the mem
function get(c, pos, mode::Mode)
	if mode == Position
		#probably won't happen? Position arg out of bounds returns pos = 0 to lookup
		pos = pos > length(c.mem) ? 0 : c.mem[pos]
		get(c, pos+1, Immediate)
	elseif mode == Immediate
		#return the value at the arg position
		pos > length(c.mem) ? 0 : c.mem[pos]
	elseif mode == Relative
		# add base to arg position value
		pos = pos > length(c.mem) ? 0 : c.mem[pos]
		get(c, pos + 1 + c.base, Immediate)
	end
end

#Write is either in position mode or relative mode
function set!(c, pos, value, mode::Mode)
	pos = c.mem[pos] + 1 + (mode == Relative ? c.base : 0)
	for i in length(c.mem):pos
		push!(c.mem, 0)
	end
	c.mem[pos] = value
end

# the opcode implementations directly modify the memory vector
# pos is the location of the opcode, parameters start at pos+1
# p1 .. p3 indicate the parameter mode (position/immediate/relatibe), not all
# are required, but all are provided.
function add!(c, pos, p1, p2, p3)
	set!(c, pos+3, get(c, pos+1, p1) + get(c, pos+2, p2), p3)
	4
end

function mul!(c, pos, p1, p2, p3)
	set!(c, pos+3, get(c, pos+1, p1) * get(c, pos+2, p2), p3)
	4
end

#save can now be in either position or relative mode
#I'm not sure about the other write operations
function save!(c, pos, p1, p2, p3)
	set!(c, pos+1, take!(c.input), p1)
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
		set!(c, pos+3, 1, p3)
	else
		set!(c, pos+3, 0, p3)
	end
	4
end

function equals!(c, pos, p1, p2, p3)
	if get(c, pos+1, p1) == get(c, pos+2, p2)
		set!(c, pos+3, 1, p3)
	else
		set!(c, pos+3, 0, p3)
	end
	4
end

function setbase!(c, pos, p1, p2, p3)
	c.base += get(c, pos+1, p1)
	2
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

const opcodes = Dict(1=>add!, 2=>mul!, 3=>save!, 4=>print, 5=>jift, 6=>jiff, 7=> lessthan!, 8=>equals!, 9=>setbase!)

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
	close(comp.output)
	if isready(comp.output)
		fetch(comp.output)
	end
end

export Computer, run!, put_input!, take_output!, has_output

end
