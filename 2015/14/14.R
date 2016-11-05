distance <- function(speed, dur, rest, time) {
	dist <- floor(time/(dur+rest))*dur*speed
	remaining <- time %% (dur+rest)
	dist <- dist + if(remaining <= dur) remaining*speed else dur*speed
	dist
}

read.delim("14_in.txt", sep=" ", header=FALSE) -> d
distances <- sapply(1:nrow(d), function(r) distance(d[r,4], d[r,7], d[r,14], 2503))

cat("Max distance", max(distances), "by", as.character(d[which.max(distances),1]), "\n")

points <- rep(0, nrow(d))
for(i in 1:2503) {
	distances <- sapply(1:nrow(d), function(r) distance(d[r,4], d[r,7], d[r,14], i))
	points[which.max(distances)] <- points[which.max(distances)] + 1;
}
cat("Max points", max(points), "by", as.character(d[which.max(points),1]),"\n")
