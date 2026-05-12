let x = 0;
let y = 0;

function foo() {
	function bar() {
		console.log(x);
	}

	bar();
}

foo();
