include("../day_09/Intcode.jl")
using .Intcode

function draw(b)
	colors = [" ", "█", "▓", "═", "o"]
	image = permutedims(b)
	for i in 1:size(image,1)
		println(join(colors[image[i,:] .+ 1]))
	end
end

"run for 1 frame, get the board, coordinates of ball and paddle and score"
function frame!(c, b)
	ball = paddle = score = nothing
	while has_output(c)
		x = take_output!(c) + 1
		y = take_output!(c) + 1
		v = take_output!(c)
		if x == 0
			score = v
		else
			b[x,y] = v
			if v == 3
				paddle = (x, y)
			elseif v == 4
				ball = (x, y)
			end
		end
	end
	(ball, paddle, score)
end

function predict(ball, paddle)
	sign(ball[1] - paddle[1])
end

function play!(c, board; plot=true)
	task = @async run!(c,[0])
	yield()

	lastpaddle = nothing
	while !istaskdone(task)
		(ball, npaddle, score) = frame!(c, board)
		paddle = if !isnothing(npaddle)
			npaddle
		else
			lastpaddle
		end

		if plot
			draw(board)
			sleep(0.02)
		end
		
		move = if !isnothing(paddle)
			predict(ball, paddle)
		else
			0
		end
		lastpaddle = paddle
		put_input!(c,move)
		yield()
	end
	println("Winner!")
	(ball, npaddle, score) = frame!(c, board)
	score
end

tc = Computer("input.txt")
run!(tc)
tiles = Dict()
for (x, y, tile) in Iterators.partition(tc.output, 3)
	if tile > 0
		tiles[CartesianIndex(x,y)] = tile
	end
end

println("Day 13 part 1: There are $(sum(values(tiles) .== 2)) block tiles")


# Let's assume the board size is stable
dims = maximum(keys(tiles)) - minimum(keys(tiles)) 

screen = zeros(Int8, dims[1]+1, dims[2]+1)
arcade = Computer("input.txt")
arcade.mem[1] = 2 #Poking around like it's 1985
score = play!(arcade, screen, plot=false)
println("Finishing games yields $score points")

#turns out we don't need the screen at all..just follow the ball and win the game

