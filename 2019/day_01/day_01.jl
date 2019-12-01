using DelimitedFiles

reqfuel = m -> m > 8 ? div(m,3) - 2 : 0

function part1(masses)
	fuel = sum(map(reqfuel, masses))
end

function part2(masses)
	total = zeros(Int64, length(masses))
	fuel = map(reqfuel, masses)
	while any(f -> f > 0, fuel)
		total .+= fuel
		fuel = map(reqfuel, fuel)
	end
	sum(total)
end

@assert map(reqfuel, [12, 14, 1969, 100756]) == [2, 2, 654, 33583] "Failure in reqfuel"
@assert map(f->part2([f]), [14, 1969, 100756]) == [2, 966, 50346] "Failure in re"

masses = dropdims(readdlm("input.txt", ',', Int64), dims=2)
ans1 = @time part1(masses)
ans2 = @time part2(masses)

println("Day 1 part 1, total fuel needed: $ans1")
println("Day 1 part 2, total fuel needed: $ans2")
