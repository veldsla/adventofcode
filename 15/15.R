
read.delim("15_in.txt", header=F, sep=" ")->d
df <- data.frame(what=sub(":","",d$V1), capacity=as.numeric(sub(",", "", d$V3)), durability=as.numeric(sub(",", "", d$V5)), flavor=as.numeric(sub(",", "", d$V7)), texture=as.numeric(sub(",","", d$V9)), calories = as.numeric(sub(",","", d$V11)))


score <- function(t, n) {
	sum(replace(t, t<0, 0)) * n
}

m <- as.matrix(df[,2:5])

max <- 0;
for(s in 0:100) {
	for (p in 0:(100-s)){
		for(f in 0:(100-s-p)) {
			t <- 100 - s - p - f
			score <- c(s,p,f,t) %*% m
			tot <- prod(replace(score, score<0, 0))
			max <- max(tot, max)
		}
	}
}
print(max)

m <- as.matrix(df[,2:6])
max <- 0;
for(s in 0:100) {
	for (p in 0:(100-s)){
		for(f in 0:(100-s-p)) {
			t <- 100 - s - p - f
			if(s+p+f+t == 100) {
				score <- c(s,p,f,t) %*% m
				if(score[5] == 500) {
					tot <- prod(replace(score[1:4], score[1:4]<0, 0))
					#print(c(s,p,f,t))
					#print(score)
					max <- max(tot, max)
				}
			}
		}
	}
}
print(max)
