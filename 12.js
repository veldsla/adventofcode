//so yeah...rust. but reading json...please, can I just once. prettyplease?


var a = require("./12_in.json")

function sumit(s) {
	var sum = 0;
	if(typeof s == 'object') {
		if(Array.isArray(s)) {
			var v = s.forEach(function(e) { sum += sumit(e); });
		} else {
			Object.keys(s).forEach(function(k) { sum += sumit(s[k]); })
		}
	} else if (typeof s == 'string') {
		//do not care
	} else if (typeof s == 'number') {
		sum = s;	
	} else {
		console.log("New type ", typeof s);
	}

	return sum
}

function sumit_nored(s) {
	var sum = 0;
	if(typeof s == 'object') {
		if(Array.isArray(s)) {
			var v = s.forEach(function(e) { sum += sumit_nored(e); });
		} else {
			var osum = 0;
			var keep = true;
			Object.keys(s).forEach(function(k) { if (s[k] === 'red') { keep = false; } osum += sumit_nored(s[k]); })
			if(keep) sum += osum;
		}
	} else if (typeof s == 'string') {
		//do not care
	} else if (typeof s == 'number') {
		sum = s;	
	} else {
		console.log("New type ", typeof s);
	}

	return sum
}

console.log("Sum is ", sumit(a));
console.log("Sum no red is ", sumit_nored(a));
