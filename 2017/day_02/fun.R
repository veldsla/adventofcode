d <- read.delim("input.txt", header=F)
sum(apply(d,1,function(x) diff(range(x))))
sum(sapply(1:nrow(d), function(row) apply(apply(combn(ncol(d), 2),2,function(col) sort(c(d[row, col[1]], d[row, col[2]]))), 2, function(p) if (p[2] %% p[1] == 0) { p[2] / p[1] } else { 0 } )))

