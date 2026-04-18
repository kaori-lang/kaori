const start = performance.now(); // milliseconds

const iterations = 1_000_0000;
const max_fib = 30;

for (let i = 0; i < iterations; i++) {
	let a = 0,
		b = 1;
	for (let j = 0; j < max_fib; j++) {
		const temp = a + b;
		a = b;
		b = temp;
	}
}

const elapsed = performance.now() - start;
console.log(elapsed);
