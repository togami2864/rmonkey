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

let twice = fn(f, x) { return f(f(x));};
let addTwo = fn(x) { return x + 2;};
twice(addTwo, 2);