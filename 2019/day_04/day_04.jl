const DPOW = [100000, 10000,1000,100,10,1]

function digit(num::Int64, pos)
	num ÷ DPOW[pos] % 10
end

function is_valid(pw)
	last =  digit(pw, 1)
	double = false
	for i in 2:6
		d = digit(pw, i)
		if d == last
			double = true
		end
		if d < last
			# in principle we can  now skip all pw tests
			# from repeat last digit till end
			return false
		end
		last = d

	end
	double
end


function find_pw(low, high)
	valid = []
	for pw in low:high
		if is_valid(pw)
			push!(valid, pw)
		end
	end
	valid
end

function isolated_pairs(pw)
	last = digit(pw, 1)
	count = 1
	for i in 2:6
		d = digit(pw, i)
		if d == last
			count += 1
		elseif count == 2
			return true
		else
			count = 1
			last = d
		end
	end
	count == 2
end

@assert is_valid(111111)
@assert !is_valid(223450)
@assert !is_valid(123789)

@assert isolated_pairs(112233)
@assert !isolated_pairs(123444)
@assert isolated_pairs(111122)

part1 = find_pw(197487, 673251)
println("Day 4 part 1: Found $(length(part1)) valid passwords")

part2 = filter(isolated_pairs, part1)
println("Day 4 part 2: Found $(length(part2)) valid passwords")






