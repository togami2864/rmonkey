# rmonkey
> Rust implementation of the programming language **Monkey** that lives in books

## Features
- C-like Syntax
- Variable bindings
- Integer and Boolean
- Built-in data structures(String, Array, Hash)
- Arithmetic expressions
- If expressions
- Built-in functions
- First-class and higher-order functions
- Support closures

## overview
- [Literal](#Literal)
    - [Integer](#Integer)
    - [Boolean](#Boolean)
    - [String](#String)
    - [Array](#Array)
    - [Hash](#Hash)
    - [Function](#Function)
- [Built-in functions](#built-in-functions)

### Literal

#### Integer
`Integer` represents **2^-63**(-9223372036854775808) to **2^63**(9223372036854775807).
##### example
```
1;
-1;
101;
9223372036854775807;
```

#### Boolean
`Boolean` represents only two possible value, **true** or **false**.
##### example
```
true;
false;
let flag = !true;
```

#### String
`String` represents the array of characters.Double-Quoted string is only available.
##### example
```
"Hello World"
"foo" + "bar" + "baz"
```

#### Array
`Array` represents a collection of elements.
###### example
```
let array = [1, 2, 3, 4, 5]
array[0] // 1
array[1+1] // 3
```
```
let array = [1, "foo", true, fn(a){a}]
array[4](2)
```

#### Hash
`Hash` represents the data structure that holds a pair of key and value.
##### example
```
let hash = {"foo":1, "bar": 2, "baz": "hello", 100: "world"};
hash["foo"]
hash[99 + 1]
```
#### Function
##### example
```
fn add(x, y){
    return x + y;
}
```
```
let fibonacci = fn(x) {
  if (x == 0) {
    0;
  } else {
    if (x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};
fibonacci(5);
```
```
let twice = fn(f, x) { return f(f(x));};
let addTwo = fn(x) { return x + 2;};
twice(addTwo, 2);
```

### Built-in functions
#### `puts(<arg1>, <arg2>, ...): void`
```
puts("string");
puts(2);
```
#### `len(<arg>): Integer`
```
len([0,1,2,3,4])
len("Hello World")
```
#### `first(<arg>): any`
```
first([0,1,2,3]) // => 0
first(["string", 0, true]) // "string"
```
#### `last(<arg>): any`
```
last([0,1,2,3,4]) // => 4
```
#### `rest(<arg>): Array`
```
rest([0,1,2]) // => [1,2]
```
#### `push(<arg1>, <arg2>): Array`
```
push([0,1,2,3,4], 5)
```