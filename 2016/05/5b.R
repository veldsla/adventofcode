library(openssl)
n<- 0
pass <- rep(NA_character_,8)
repeat {
	cat("N =", n, "\n")
	hashes <- md5(paste0("cxdnnyjw",seq.int(n, n+1000000)))
	n <- n + 1000000
	u <- which(sapply(hashes, substr ,0,5) == "00000")
	for (chars in strsplit(hashes[u], "", fixed=T)) {
		pos <- as.numeric(chars[6]) + 1
		if (!is.na(pos) && pos >= 1 && pos <= 8 && is.na(pass[pos])) {
			pass[pos] = chars[7]
		}
	}
	if (any(is.na(pass))) next
	break
}
cat("The password = ", paste(pass, collapse=""),"\n")
