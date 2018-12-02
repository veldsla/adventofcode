read.delim("input.txt", header=F, stringsAsFactors=F)[,1]->d
strsplit(d,"") ->dd
lapply(dd, table) -> counts
twos <- sapply(counts,function(c) any(c==2))
threes <- sapply(counts,function(c) any(c==3))
# 2a
sum(twos) * sum(threes)

library(stringdist)
i <- which(sapply(d, function(s) any(stringdist(s, d, method="hamming")==1)))
# 2b
paste(dd[[i[1]]][dd[[i[1]]] %in% dd[[i[2]]]], collapse="")
