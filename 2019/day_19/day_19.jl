include("../day_09/Intcode.jl")
using .Intcode	

function draw(b)
	colors = [".", "█","O","X", "S"]
	image = b;# permutedims(b)
	for i in 1:size(image,1)
		println(join(colors[image[i,:] .+ 1]))
	end
end

function beamscan(c, wh=50,ox=0, oy=0)
	m = zeros(Int8, wh, wh)
	for (x, y) in Iterators.product(0:wh-1, 0:wh-1)
		m[x+1, y+1] = run!(copy(c), [x+ox, y+oy])
	end
	m
end


function testwh(c, x, y)
	@assert run!(copy(c), [x, y]) == 1
	for i in 1:99
		if run!(copy(c), [x+i, y-i]) != 1
			return false
		end
	end
	true
end

function followbeamto100(c, x, y)
	@assert run!(copy(c), [x, y]) == 1

	while true
		#move right and down until no beam
		x+=1
		while run!(copy(c),[x, y]) == 1
			y+=1
		end
		y -=1
		if testwh(c, x, y)
			return (x, y-99)
		end
	end
end

c = Computer("input.txt")
area = beamscan(c)
draw(area)
println("Day 19 part 1: Tractor beam pulls in $(sum(area .== 1)) points")
#start anywhere in the beam
findall(area.==1)[end-20:end]
# CartesianIndex(42, 50)
tl = followbeamto100(c, 42, 50)
# tl is zero based
area = beamscan(c,120,tl[1], tl[2])
#add space ship (area is 1-based)
for x in 1:100, y in 1:100
	if area[x,y] == 1
		area[x,y]=2
	else
		area[x,y] = 3
	end
end
area[1,1] = 4
draw(area)

println("Day 19 part 2: Tractor beam fits at $tl, answer is $(tl[1]*10000+tl[2])")

