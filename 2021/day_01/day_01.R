input <- read.delim("input.txt", header=F)[,1]
#1a
sum(diff(input) >0)
#1b
sum(diff(filter(input, c(1,1,1)))>0, na.rm=TRUE)
