
walk <- function(maze, plotpoint=NULL) {
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
		if (!is.null(plotpoint)) {
			plotpoint(row, col, maze[row, col])
		}

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

mazeplotter <- function(m) {
	x11(width=10, height=10)
	par(mar=c(1,1,1,1))
	image(1:nrow(m), 1:ncol(m), z=ifelse(t(m)[,nrow(m):1]==" ",0,1), col=c("white", "gray"), xlab="", ylab="", xaxt="n", yaxt="n", bty="n", main="AoC 2017 - day 19")
	x <- 0
	#the point plotter
	function(row, col, letter=NULL) {
		x<<- x + 1
		points(col, ncol(m) + 1 - row, pch=15, col="red", cex=.7)
		if (letter %in% LETTERS) text(col, ncol(m) + 1 - row, col="blue", label=letter, cex=1, font=2)
	}
}

#load data
m <- t(simplify2array(strsplit(readLines("input.txt"),"")))

# the stuff for the animation
fun <- mazeplotter(m)

#walk the line
#print(walk(m))

#or with plotted route:
print(walk(m, plot=fun))

