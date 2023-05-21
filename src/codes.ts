export const sampleCodes = [
  {
    text: 'integers',
    value: `1;
-1;
101;
9223372036854775807;`,
  },
  {
    text: 'boolean',
    value: `true;
false;
let flag = !true;`,
  },
  {
    text: 'string',
    value: `"Hello World"
"foo" + "bar" + "baz"`,
  },
  {
    text: 'array',
    value: `let array = [1, 2, 3, 4, 5]
array[0]
array[1+1]
let array = [1, "foo", true, fn(a){a}]
array[4](2)
`,
  },
  {
    text: 'hash',
    value: `let hash = {"foo":1, "bar": 2, "baz": "hello", 100: "world"};
hash["foo"]
hash[99 + 1]`,
  },
  {
    text: 'function',
    value: `fn add(x, y){
    return x + y;
}`,
  },
  {
    text: 'high-order function',
    value: `let twice = fn(f, x) { return f(f(x));};
let addTwo = fn(x) { return x + 2;};
twice(addTwo, 2);`,
  },
  {
    text: 'fibonacci',
    value: `let fibonacci = fn(x) {
  if(x == 0) {
    0;
  } else {
    if(x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    };
  };
};
fibonacci(5);`,
  },
];
