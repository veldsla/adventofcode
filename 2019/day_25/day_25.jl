include("../day_09/Intcode.jl")
using .Intcode	
using Combinatorics

ascii = s -> map(i->convert(Int, s[i]), 1:length(s))
function put_ascii!(c, s)
	for v in ascii(s)
		put_input!(c, v)
	end
	put_input!(c,10)
end

function read_output(c)
	s = ""
	while has_output(c)
		s = string(s, Char(take_output!(c)))
	end
	s
end

function playrepl!(c)
	t = @async run!(c)
	yield()
	println(read_output(c))
	while !istaskdone(t)
		@show command = readline()
		global backup = c
		put_ascii!(c, command)
		yield()
		print(read_output(c))
	end
end

function crack!(c, p)
	t = @async run!(c)
	yield()
	print(read_output(c))
	items = []
	for command in split(p, "\n")
		if startswith(command, "take ")
			push!(items, command[6:end])
		end
		put_ascii!(c, command)
		yield()
		print(read_output(c))
	end
	#drop all
	for i in items
		put_ascii!(c, "drop $i")
		yield()
		print(read_output(c))
	end

	exit = false
	while !exit
		for l in 2:length(items)
			for collection in combinations(items, l)
				#drop all items

				for i in collection
					put_ascii!(c, "take $i")
					yield()
					read_output(c)
				end
				put_ascii!(c, "east")
				yield()
				res = read_output(c)
				print(res)

				if !occursin("and you are ejected back to the checkpoint", res)
					println("This worked: ", join(collection, ", "))
					exit = true
					break
				end
				for i in collection
					put_ascii!(c, "drop $i")
					yield()
					print(read_output(c))
				end
			end
			if exit
				break
			end
		end
	end
end


c = Computer("input.txt")
#playrepl!(c)

#after some nice playing we walk the following route and take some items
#then we brute force the exit
program = """
east
take antenna
east
take ornament
north
west
take fixed point
east
south
west
north
north
take asterisk
south
west
west
take astronaut ice cream
east
south
take hologram
north
east
south
west
south
south
south
take dark matter
north
west
north
take monolith
north
north
inv"""

crack!(c, program)

