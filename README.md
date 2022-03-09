# Kagami

## Syntax

### module
Every file is a module, the module name is the same as the file name.

### function
There are two types of `function`:  
- normal function defined in a Kagami module
```
func Arubaito(a: int, {b: String, c: bool?}): int {
	// ...
	return 100;
}
```
- native function (provided by the `Kagami Runtime`)
```
func readFile(path: String): String = native "io_read_file";
```

### let
```
let a: String?; // a is null
let b = 123; // the type of 'b' is int
a = "kagami";
```

### when
```
when (b) {
	1 => {
		// ...
	},
	2 => {
		// ...
	},
	else => {
		// ...
	},
}
```

### for
```
for (let a in array) {
	// ...
}
```

### while
```
while (b > 100) {
	b -= 1;
}
```