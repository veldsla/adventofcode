include("../day_09/Intcode.jl")
using .Intcode	

ascii = s -> map(i->convert(Int, s[i]), 1:length(s))
function put_ascii!(c, s)
	for v in ascii(s)
		put_input!(c, v)
	end
	put_input!(c,10)
end


function jumprun!(c,p)
	t = @async run!(c)
	for l in split(p, "\n")
		@show l
		put_ascii!(c, l)
	end
	put_ascii!(c, "WALK")

	d = collect(c.output)
	#println(join(map(x->Char(x), d)))
	d
end

c = Computer("input.txt")

prog = """
NOT C J
AND D J
NOT A T
OR T J"""
jumprun!(copy(c), prog)

prog2 = """
OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN"""
jumprun!(copy(c), prog2)



