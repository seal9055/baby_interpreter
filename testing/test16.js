function add(x, y) {
	return (x + y);
}

function sub(x, y) {
	return (x - y);
}

function math(x, y) {
	return add(sub(x, 3), add(y, 4));	
}

console.log(math(10, 3));
