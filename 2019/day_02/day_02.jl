# intcode has 0-based integers, julia 1-based :-(

function add!(v, pos)
	v[v[pos+3]+1] = v[v[pos+1]+1] + v[v[pos+2]+1];
end

function mul!(v, pos)
	v[v[pos+3]+1] = v[v[pos+1]+1] * v[v[pos+2]+1];
end

opcodes = Dict(1=>add!, 2=>mul!)

function run!(code)
	pos = 1
	while haskey(opcodes, code[pos])
		opcodes[code[pos]](code, pos)
		pos += 4
	end
	code
end
	
@assert run!([1,9,10,3,2,3,11,0,99,30,40,50]) == [3500,9,10,70,2,3,11,0,99,30,40,50] "Error in intcode runner p1-1"
@assert run!([2,3,0,3,99]) == [2,3,0,6,99] "Error in intcode runner p1-2"
@assert run!([2,4,4,5,99,0]) == [2,4,4,5,99,9801] "Error in intcode runner p1-3"
@assert run!([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99] "Error in intcode runner p1-4"

intcode = parse.(Int64, split(readline("input.txt"), ','))
part1 = copy(intcode)
part1[2] = 12
part1[3] = 2
run!(part1)
println("Day 2 part 1: Value at pos 0 = ", part1[1])

for (noun, verb) in Iterators.product(0:99, 0:99)
	mem = copy(intcode)
	mem[2] = noun
	mem[3] = verb
	run!(mem)
	if mem[1] == 19690720
		println("Day 2 part 2: Value 19690720 reached at noun=$noun, verb=$verb, answer is $noun$verb")
		break
	end
end


