const width = 800;
const height = 800;
const maxIter = 1000;

const xMin = -2.5;
const xMax = 1.0;
const yMin = -1.25;
const yMax = 1.25;

for (let y = 0; y < height; y++) {
	for (let x = 0; x < width; x++) {
		const cx = xMin + (x * (xMax - xMin)) / width;
		const cy = yMin + (y * (yMax - yMin)) / height;

		let zx = 0.0;
		let zy = 0.0;

		let iter = 0;

		while (zx * zx + zy * zy < 4.0 && iter < maxIter) {
			const xtemp = zx * zx - zy * zy + cx;

			zy = 2.0 * zx * zy + cy;
			zx = xtemp;

			iter++;
		}
	}
}
