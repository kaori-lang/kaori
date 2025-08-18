const startTime = performance.now();

function fib(n) {
	if (n === 0) return 0;
	if (n === 1) return 1;

	return fib(n - 1) + fib(n - 2);
}

console.log(fib(40));

const endTime = performance.now();
const executionTime = (endTime - startTime) / 1000; // convert ms to seconds

console.log(`Total time: ${executionTime.toFixed(3)}s`);
