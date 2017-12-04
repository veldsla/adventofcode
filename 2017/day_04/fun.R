s <- lapply(scan(file="input.txt", what=character(), sep="\n", multi.line=F, flush=T), function(w) strsplit(w," ")[[1]])
#4a
sum(sapply(s, function(w) sum(duplicated(w)) == 0))

#4b
isanagram <- function(a, b) isTRUE(all.equal(sort(strsplit(a,"")[[1]]), sort(strsplit(b,"")[[1]])))
sum(sapply(s, function(w) sum(duplicated(w)) == 0 && all(apply(combn(w,2), 2, function(p) !isanagram(p[1], p[2]))))
