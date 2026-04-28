function main() {
	let foo = (n) => {
		if (n < 0) return;
		bar(n - 1);
	};

	let bar = (n) => {
		console.log(x);
		if (n < 0) return;
		foo(n - 1);
	};

	let x = 7;
	foo(10);
	console.log("end");
}

main();
