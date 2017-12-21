rotate <- function(m) t(apply(m, 2, rev))
flip_h <- function(m) apply(m,2,rev)
flip_v <- function(m) apply(m,1,rev)
as_str <- function(m) paste(as.character(t(m)), collapse="")
as_mat <- function(s) matrix(strsplit(s, "")[[1]], nrow=sqrt(nchar(s)), byrow=TRUE)

readrules <- function(f) {
	l <- readLines(f)
	rules <- list()
	data <- strsplit(l, " => ")
	for (rule in data) {
		m <- lapply(strsplit(rule, "/"), paste, collapse="")
		from <- m[[1]][1]
		to <- m[[2]][1]

		from_m <- as_mat(from)
		to_m <- as_mat(to)
		for(i in 1:4) {
			from_m = rotate(from_m)
			rules[[as_str(flip_v(from_m))]] <- to_m
			rules[[as_str(flip_h(from_m))]] <- to_m
			rules[[as_str(flip_v(from_m))]] <- to_m
		}
	}
	rules
}

do <- function(it=5) {
	art <- ".#...####"
	m <- as_mat(art)
	for (i in 1:it) {
		size <- ncol(m)
		if (size %% 2 == 0) {
			#make submatrices of 2x2 and explode
			newart <- matrix("", ncol=(size / 2) * 3, nrow=(size / 2) * 3)
			for (x in seq(1, size, 2)) {
				for (y in seq(1, size, 2)) {
					ex <- rules[[as_str(m[x:(x+1), y:(y+1)])]]
					newx <- seq.int(floor(x/2*3), length.out=3)
					newy <- seq.int(floor(y/2*3), length.out=3)
					newart[newx, newy] <- ex
				}
			}
			m <- newart
		} else {
			#make submatrices of 3x3 and explode
			newart <- matrix("", ncol=(size / 3) * 4, nrow=(size / 3) * 4)
			for (x in seq(1, size, 3)) {
				for (y in seq(1, size, 3)) {
					ex <- rules[[as_str(m[x:(x+2), y:(y+2)])]]
					newx <- seq.int(floor(x/3*4), length.out=4)
					newy <- seq.int(floor(y/3*4), length.out=4)
					newart[newx, newy] <- ex
				}
			}
			m <- newart
		}
	}
	sum(m=="#")
}

rules <- readrules("input.txt");
cat("21a:", do(it=5), "are on after 5 iterations\n")
cat("21b:", do(it=18), "are on after 18 iterations\n")
