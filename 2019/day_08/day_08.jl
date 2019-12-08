data = parse.(Int8, split(readline("input.txt"),""))

w = 25
h = 6
nlayers = length(data) ÷ (w * h)

m = reshape(data, (w,h,nlayers))

minzerolayer = argmin(map(l -> sum(m[:,:,l] .== 0), 1:nlayers))
ans1 = sum(m[:,:,minzerolayer] .== 1) * sum(m[:,:,minzerolayer] .== 2)
println("Day 8 part 1: Number of ones multiplied with number of twos in layer with minimum number of zeros($minzerolayer) = $ans1")

pixelcol = v -> first(filter(p -> p < 2, v))
image = fill(" ", (h, w))
colors = ["█", "░", " "]
for x in 1:w, y in 1:h
	image[y,x] = colors[pixelcol(m[x,y,:])+1]
end

println("Day 8 part 2: Decoded images below:")
for i in 1:h
	println(join(image[i,:]))
end

