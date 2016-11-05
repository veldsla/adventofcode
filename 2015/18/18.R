neighbours <- function(p, nrow=100, ncol=100) {
	coords <- numeric(0);

	up   <-  (p-1) %% nrow != 0
	down <-  p %% nrow != 0
	left <- p > nrow
	right<- p <= ((nrow*ncol) - nrow)

	if(up) coords <- c(coords,p-1)
	if(left) coords <- c(coords,p-nrow)
	if(right) coords <- c(coords,p+nrow)
	if(down) coords <- c(coords,p+1)
	if(up && left)  coords <- c(coords,p-1-nrow)
	if(up && right)  coords <- c(coords,p-1+nrow)
	if(down && left)  coords <- c(coords,p+1-nrow)
	if(down && right)  coords <- c(coords,p+1+nrow)

	coords
}

read.delim("18_in.txt", header=F, stringsAsFactors=F) ->d
sapply(strsplit(d[,1],""), function(r) ifelse(r=="#",1,0)) -> mat
orimat <- mat
#precalculate neighbours
n <- lapply(1:prod(dim(mat)), function(i) neighbours(i, nrow=nrow(mat), ncol=ncol(mat)))

for(i in 1:100) {
	numon <- sapply(n, function(cells) sum(mat[cells]))
	off <- which(mat == 1 & (numon != 2 & numon != 3))
	on <- which(mat == 0 & numon == 3)
	mat[off] <- 0
	mat[on] <- 1
}
cat("18a: There are", sum(mat), "lights on\n")

#18b
mat <- orimat
mat[1,1] <- 2
mat[100,1] <- 2
mat[1,100] <- 2
mat[100,100] <- 2

for(i in 1:100) {
	numon <- sapply(n, function(cells) sum(mat[cells]>0))
	off <- which(mat == 1 & (numon != 2 & numon != 3))
	on <- which(mat == 0 & numon == 3)
	mat[off] <- 0
	mat[on] <- 1
}
cat("18b: There are", sum(mat)-4, "lights on\n")

