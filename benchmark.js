const n = 100_000_007;
const startTime = performance.now();

let isPrime = true;

for (let i = 2; i < n; i++) {
	if (n % i === 0) {
		console.log("no");
		isPrime = false;
		break;
	}
}

const endTime = performance.now();
const executionTime = (endTime - startTime) / 1000; // convert ms to seconds

if (isPrime) {
	console.log("yes");
}

console.log(`Total time: ${executionTime.toFixed(3)}s`);
