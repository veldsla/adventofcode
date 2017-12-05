d <- read.delim("input.txt", header=F)[,1]
p <- 1
n <- 0
while (p > 0 && p <= length(d)) {
	o <- d[p]
	d[p] = d[p] + 1
	p <- p + o
	n <- n + 1
}
cat("5a: Maze exit in", n, "steps\n")

d <- read.delim("input.txt", header=F)[,1]
p <- 1
n <- 0
while (p > 0 && p <= length(d)) {
	o <- d[p]
	if (o >= 3) {
		d[p] = d[p] - 1
	} else {
		d[p] = d[p] + 1
	}
	p <- p + o
	n <- n + 1
}
cat("5b: Maze exit in", n, "steps\n")

# optimization probably possible. All negavtive jumps are at the end. Biggest being the last (-681)
# postive values will toggle between 3 and 4 mostly.

