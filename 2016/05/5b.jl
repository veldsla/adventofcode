using Nettle

tic()
input = "cxdnnyjw"
h = Hasher("md5")

function next_from(seed, n)
	while true
		update!(h, "$seed$n")
		hv = hexdigest!(h)
		n += 1
		if hv[1:5] == "00000"
			return hv, n
		end
	end
end

pass = Dict{Char,Char}()
n = 0
while length(pass) != 8
	hv, n = next_from(input, n)
	if hv[6] >= '0' && hv[6] <= '7' && !haskey(pass,hv[6])
		pass[hv[6]] = hv[7]
	end
end

toc()
