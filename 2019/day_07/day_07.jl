using Combinatorics
include("Intcode.jl")
using .Intcode

function find_phase(pc)
	perms = permutations(0:4)
	highest = 0
	for p in perms
		phases = [copy(pc) for x in 1:5]
		out = 0
		for i in 1:5
			out = run!(phases[i], [p[i], out])
		end
		highest = max(highest, out)
	end
	highest
end

function find_phase_feedback(pc)
	perms = permutations(5:9)
	highest = 0
	for p in perms
		phases = [copy(pc) for x in 1:5]
		tasks = []
		for i in 1:5
			push!(tasks, @async run!(phases[i], [p[i]]))
		end
		
		# this println makes it work an so does yield
		# do not understand why
		#println("highest= $highest")
		yield()
		

		while !istaskdone(tasks[5])
			v5 = has_output(phases[5]) ? take_output!(phases[5]) : 0
			put_input!(phases[1], v5)
			v1 = take_output!(phases[1])
			put_input!(phases[2], v1)
			v2 = take_output!(phases[2])
			put_input!(phases[3], v2)
			v3 = take_output!(phases[3])
			put_input!(phases[4], v3)
			v4 = take_output!(phases[4])
			put_input!(phases[5], v4)
			wait(phases[5].output)
		end
		highest = max(highest, take_output!(phases[5]))
	end
	highest
end


@assert find_phase(Computer([3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0])) == 43210
@assert find_phase_feedback(Computer([3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5])) == 139629729
@assert find_phase_feedback(Computer([3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10])) == 18216

c = Computer("input.txt")
h = find_phase(c);
println("Day 7 part 1: Highest output is $h")

h = find_phase_feedback(c);
println("Day 7 part 2: Highest output is $h")


