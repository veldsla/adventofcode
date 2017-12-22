
walk <- function(maze) {
	word <- ""
	dir <- c(1,0)
	dcol <- 0
	seen <- matrix(FALSE, nrow=nrow(maze), ncol=ncol(maze))
	steps <- 0

	row <- 1
	col <- min(which(maze[1,] == "|"))

	while(maze[row, col] != " ") {
		steps <- steps + 1
		seen[row, col] <- TRUE

		#walk until we see a +
		if (maze[row, col] == "+") {
			#turn right if we haven't already
			newd <- turnright(dir)
			if (maze[row + newd[1] , col + newd[2]] == " " || seen[row + newd[1] , col + newd[2]]) {
				#turn left 
				newd<- turnleft(dir)
			}
			dir<- newd

		} else if (maze[row, col] %in% LETTERS) {
			word <- paste0(word, m[row, col])
		}
		row <- row + dir[1]
		col <- col + dir[2]
	}
	list(word=word, steps=steps)
}

turnright <- function(d) {
	if (d[1] == 0) {
		c(d[2],0)
	} else {
		c(0,-d[1])
	}
}

turnleft <- function(d) {
	if (d[1] == 0) {
		c(-d[2],0)
	} else {
		c(0,d[1])
	}
}


#load data
m <- t(simplify2array(strsplit(readLines("input.txt"),"")))

#walk the line
print(walk(m))

