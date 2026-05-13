const width = 800;
const height = 800;
const max_iteration = 1000;
const x_min = -2.5;
const x_max = 1.0;
const y_min = -1.25;
const y_max = 1.25;

for (let y = 0; y < height; y++) {
	for (let x = 0; x < width; x++) {
		const cx = x_min + (x * (x_max - x_min)) / width;
		const cy = y_min + (y * (y_max - y_min)) / height;
		let zx = 0.0;
		let zy = 0.0;
		let iteration = 0;
		while (zx * zx + zy * zy < 4.0 && iteration < max_iteration) {
			const xtemp = zx * zx - zy * zy + cx;
			zy = 2.0 * zx * zy + cy;
			zx = xtemp;
			iteration++;
		}
	}
}
