let vec = [0, 1, 2];

let foo = { bar: vec };

foo.bar[2] = "foo";

console.log(vec);
