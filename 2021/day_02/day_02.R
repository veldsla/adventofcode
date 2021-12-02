d <- read.delim("input.txt", sep=" ", header=F, col.names=c("dir", "amount"))

s <- tapply(d$amount, d$dir, sum)
#1a
s["forward"] * (s["down"] - s["up"])


v <- c(forward=0, up=-1, down=1)
aim <- cumsum(v[d$dir] * d$amount)
d2 <- sum(d$amount[d$dir == "forward"] * aim[d$dir == "forward"])

#2a
s["forward"] * d2


