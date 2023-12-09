rd <- function(v) {
	n <- v
	last <- numeric()
	while(any(n != 0) && length(n) > 1) {
		n <- diff(n)
		last <- c(last, tail(n, 1))
	}
	sum(last) + tail(v, 1)
}

rd2 <- function(v) {
	n <- v
	first <- numeric()
	while(any(n != 0) && length(n) > 1) {
		n <- diff(n)
		first <- c(first, head(n, 1))
	}
	#print(first)

	head(v,1) - cmsum(rev(first))
}

cmsum <- function(v) {
	sum = -v[1]
	for (i in 2:length(v)) {
		sum = v[i] - sum
	}
	sum
}

m <- read.delim("inputs/day_09.txt", sep=" ", header=FALSE)
m <- as.matrix(m)

#part 1
sum(apply(m, 1, rd))
#part 2
sum(apply(m, 1, rd2))


