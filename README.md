# math-as-rust ๐ฆ

>Based on [math-as-code](https://github.com/Jam3/math-as-code/blob/master/rust-README.md)

This is a reference to ease developers into mathematical notation by showing comparisons with Rust code.

Motivation: Academic papers can be intimidating for self-taught graphics programmers and data wranglers :)

This guide is not yet finished. If you see errors or want to contribute, please [open a ticket](https://github.com/eduardonunesp/math-as-rust/issues) or send a PR.

## foreword

Mathematical symbols can mean different things depending on the author, context and the field of study (linear algebra, set theory, etc). This guide may not cover *all* uses of a symbol. In some cases, real-world references (blog posts, publications, etc) will be cited to demonstrate how a symbol might appear in the wild.

For a more complete list, refer to [Wikipedia - List of Mathematical Symbols](https://en.wikipedia.org/wiki/List_of_mathematical_symbols).

For simplicity, many of the code examples here operate on floating point values and are not numerically robust. For more details on why this may be a problem, see [Robust Arithmetic Notes](https://github.com/mikolalysenko/robust-arithmetic-notes) by Mikola Lysenko.

## contents

- [variable name conventions](#variable-name-conventions)
- [equals `=` `โ` `โ ` `:=`](#equals-symbols)
- [square root and complex numbers `โ` *`i`*](#square-root-and-complex-numbers)
- [dot & cross `ยท` `ร` `โ`](#dot--cross)
  - [scalar multiplication](#scalar-multiplication)
  - [vector multiplication](#vector-multiplication)
  - [dot product](#dot-product)
  - [cross product](#cross-product)
- [sigma `ฮฃ`](#sigma) - *summation*
- [capital Pi `ฮ `](#capital-pi) - *products of sequences*
- [pipes `||`](#pipes)
  - [absolute value](#absolute-value)
  - [Euclidean norm](#euclidean-norm)
  - [determinant](#determinant)
- [hat **`รข`**](#hat) - *unit vector*
- ["element of" `โ` `โ`](#element)
- [common number sets `โ` `โค` `โ` `โ`](#common-number-sets)
- [function `ฦ`](#function)
  - [piecewise function](#piecewise-function)
  - [common functions](#common-functions)
  - [function notation `โฆ` `โ`](#function-notation)
- [prime `โฒ`](#prime)
- [floor & ceiling `โ` `โ`](#floor--ceiling)
- [arrows](#arrows)
  - [material implication `โ` `โ`](#material-implication)
  - [equality `<` `โฅ` `โซ`](#equality)
  - [conjunction & disjunction `โง` `โจ`](#conjunction--disjunction)
- [logical negation `ยฌ` `~` `!`](#logical-negation)
- [intervals](#intervals)

## variable name conventions

There are a variety of naming conventions depending on the context and field of study, and they are not always consistent. However, in some of the literature you may find variable names to follow a pattern like so:

- *s* - italic lowercase letters for scalars (e.g. a number)
- **x** - bold lowercase letters for vectors (e.g. a 2D point)
- **A** - bold uppercase letters for matrices (e.g. a 3D transformation)
- *ฮธ* - italic lowercase Greek letters for constants and special variables (e.g. [polar angle *ฮธ*, *theta*](https://en.wikipedia.org/wiki/Spherical_coordinate_system))

This will also be the format of this guide.

### Rust Math Lib?

## equals symbols

There are a number of symbols resembling the equals sign `=`. Here are a few common examples:

- `=` is for equality (values are the same)
- `โ ` is for inequality (value are not the same)
- `โ` is for approximately equal to (`ฯ โ 3.14159`)
- `:=` is for definition (A is defined as B)

In Rust:

```rust
// equality
2 == 3

// inequality
2 != 3

// approximately equal
#[macro_use]
extern crate is_close;

// is_close! doesn't have a third argument for tolerance, so this is false
is_close!(std::f64::consts::PI, 3.14159), true)

fn is_almost_equal(x: f64, y: f64, epsilon: f64) -> bool {
    (x - y).abs() < (10f64.powf(-epsilon))
}

is_almost_equal(std::f64::consts::PI, 3.14159, 1e-5) // true
```

> **Read more**: programmers got this idea from the [epsilon-delta definition of limit][1]

In mathematical notation, you might see the `:=`, `=:` and `=` symbols being used for *definition*.<sup>[2]</sup>

For example, the following defines *x* to be another name for 2*kj*.

![equals1](http://latex.codecogs.com/svg.latex?x%20%3A%3D%202kj)

<!-- x := 2kj -->

In rust, we *define* our variables with `=`.

```rust
let x = 2 * k * j
```

*Assignment* in rust variables are immutable by default.

> **Note**: Some languages have pre-processor `#define` statements, which are closer to a mathematical *define*.

Notice that `fn` is a form of `:=` as well.

```rust
fn plus(x: f64, y: f64) -> f64 {
  x + y
}
```

The following, on the other hand, represents equality:

![equals2](http://latex.codecogs.com/svg.latex?x%20%3D%202kj)

<!-- x = 2kj -->

**Important**: the difference between `=` and `==` can be more obvious in code than it is in math literature! In rust, a `=` is an *instruction*. You're telling the machine to interact with the namespace, add something to it or change something in it. In rust, when you write `==` you're asking the machine "may I have a `bool`?". In math, the former case is *either* covered by `:=` or `=`, while the latter case is *usually* `=`, and you might have to do some disambiguating in your reading.

In math, when I write 1 + 1 = 2 I'm making a *judgment*. It's not that i'm asking the world (or the chalkboard) for a bool, it's that I'm keeping track of my beliefs. This distinction is the foundation of *unit tests* or *assertions*.

```rust
// assert in takes an expression that lands in bool and a string to be printed if it turns out false.
assert!(plus(1, 1) == 2, "DANGER: PHYSICS IS BROKEN. PLEASE STAY INSIDE.");
```

It's important to know when a falsehood ought to crash a program vs. when you just want a boolean value. To understand this better, read [this](https://en.wikipedia.org/wiki/G%C3%B6del,_Escher,_Bach).

## square root and complex numbers

A square root operation is of the form:

![squareroot](http://latex.codecogs.com/svg.latex?%5Cleft%28%5Csqrt%7Bx%7D%5Cright%29%5E2%20%3D%20x)

<!-- \left(\sqrt{x}\right)^2 = x -->

In programming we use a `sqrt` function, like so:

```rust
println!("{}", 2f64.sqrt());
// Out: 1.4142135623730951
```

Complex numbers are expressions of the form ![complex](http://latex.codecogs.com/svg.latex?a&space;&plus;&space;ib), where ![a](http://latex.codecogs.com/svg.latex?a) is the real part and ![b](http://latex.codecogs.com/svg.latex?b) is the imaginary part. The imaginary number ![i](http://latex.codecogs.com/svg.latex?i) is defined as:

![imaginary](http://latex.codecogs.com/svg.latex?i%3D%5Csqrt%7B-1%7D).
<!-- i=\sqrt{-1} -->

```rust
println!("{}", 2f64.sqrt());
// Out: 1+1i

use num::Complex;
let complex_integer = num::Complex::new(1, 1);
println!("{}", complex_integer.sqrt());
// Out: 1.0986841134678098+0.45508986056222733i

// we can represent the basic meaning of the imaginary unit like so
let cn1 = num::complex::Complex::new(-1, 0);
let cn2 = num::complex::Complex::new(0, 1);
assert!(cn1 == cn2); // Should fail
```

## dot & cross

The dot `ยท` and cross `ร` symbols have different uses depending on context.

They might seem obvious, but it's important to understand the subtle differences before we continue into other sections.

### scalar multiplication

Both symbols can represent simple multiplication of scalars. The following are equivalent:

![dotcross1](http://latex.codecogs.com/svg.latex?5%20%5Ccdot%204%20%3D%205%20%5Ctimes%204)

<!-- 5 \cdot 4 = 5 \times 4 -->

In programming languages we tend to use asterisk for multiplication:

```rust
let result = 5 * 4
```

Often, the multiplication sign is only used to avoid ambiguity (e.g. between two numbers). Here, we can omit it entirely:

![dotcross2](http://latex.codecogs.com/svg.latex?3kj)

<!-- 3kj -->

If these variables represent scalars, the code would be:

```rust
let result = 3 * k * j
```

#### vector multiplication

To denote multiplication of one vector with a scalar, or element-wise multiplication of a vector with another vector, we typically do not use the dot `ยท` or cross `ร` symbols. These have different meanings in linear algebra, discussed shortly.

Let's take our earlier example but apply it to vectors. For element-wise vector multiplication, you might see an open dot `โ` to represent the [Hadamard product](https://en.wikipedia.org/wiki/Hadamard_product_%28matrices%29).<sup>[2]</sup>

![dotcross3](http://latex.codecogs.com/svg.latex?3%5Cmathbf%7Bk%7D%5Ccirc%5Cmathbf%7Bj%7D)

<!-- 3\mathbf{k}\circ\mathbf{j} -->

In other instances, the author might explicitly define a different notation, such as a circled dot `โ` or a filled circle `โ`.<sup>[3]</sup>

Here is how it would look in code, using arrays `[x, y]` to represent the 2D vectors.

```rust
let s = 3
let k = vec![1, 2]
let j = vec![2, 3]

let tmp = multiply(k, j)
let result = multiply_scalar(tmp, s)
// Out: [6, 18]
```

Our `multiply` and `multiply_scalar` functions look like this:

```rust
fn multiply(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let it = a.iter().zip(b.iter());
    it.map(|(x, y)| x * y).collect()
}

fn multiply_scalar(a: Vec<i64>, scalar: i64) -> Vec<i64> {
    a.iter().map(|v| v * scalar).collect()
}
```

#### dot product

The dot symbol `ยท` can be used to denote the [*dot product*](https://en.wikipedia.org/wiki/Dot_product) of two vectors. Sometimes this is called the *scalar product* since it evaluates to a scalar.

![dotcross4](http://latex.codecogs.com/svg.latex?%5Cmathbf%7Bk%7D%5Ccdot%20%5Cmathbf%7Bj%7D)

<!-- \mathbf{k}\cdot \mathbf{j} -->

It is a very common feature of linear algebra, and with a 3D vector it might look like this:

```rust
let k = [0, 1, 0];
let j = [1, 0, 0];

let d = dot(k, j);
// Out: 0
```

The result `0` tells us our vectors are perpendicular. Here is a `dot` function for 3-component vectors:

```rust
fn dot(a, b):
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
```

#### cross product

The cross symbol `ร` can be used to denote the [*cross product*](https://en.wikipedia.org/wiki/Cross_product) of two vectors.

![dotcross5](http://latex.codecogs.com/svg.latex?%5Cmathbf%7Bk%7D%5Ctimes%20%5Cmathbf%7Bj%7D)

<!-- \mathbf{k}\times \mathbf{j} -->

In code, it would look like this:

```rust
let k = vec![0, 1, 0];
let j = vec![1, 0, 0];
let result = cross(k, j);
// Out: [ 0, 0, -1 ]
```

Here, we get `[0, 0, -1]`, which is perpendicular to both **k** and **j**.

Our `cross` function:

```rust
fn cross(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let rx = a[1] * b[2] - a[2] * b[1];
    let ry = a[2] * b[0] - a[0] * b[2];
    let rz = a[0] * b[1] - a[1] * b[0];
    vec![rx, ry, rz]
}
```

## sigma

The big Greek `ฮฃ` (Sigma) is for [Summation](https://en.wikipedia.org/wiki/Summation). In other words: summing up some numbers.

![sigma](http://latex.codecogs.com/svg.latex?%5Csum_%7Bi%3D1%7D%5E%7B100%7Di)

<!-- \sum_{i=1}^{100}i -->

Here, `i=1` says to start at `1` and end at the number above the Sigma, `100`. These are the lower and upper bounds, respectively. The *i* to the right of the "E" tells us what we are summing. In code:

Hence, the big sigma is the `std::iter::Sum` module.

```rust
std::iter::Sum::sum((0..=100).into_iter())
// Out: 5050
```

**Tip:** With whole numbers, this particular pattern can be optimized to the
following (and try to [grok the
proof](http://mathcentral.uregina.ca/QQ/database/QQ.02.06/jo1.html). The legend
of how Gauss discovered I can only describe as "typical programmer antics"):

```rust
fn sum_to_n(n: f64) -> f64 {
    (n * (n + 1.)) / 2.
}
```

Here is another example where the *i*, or the "what to sum," is different:

![sum2](http://latex.codecogs.com/svg.latex?%5Csum_%7Bi%3D1%7D%5E%7B100%7D%282i&plus;1%29)

<!-- \sum_{i=1}^{100}(2i+1) -->

In code:

```rust
std::iter::Sum::sum((0..n).map(|k| 2 * k + 1).into_iter())
// Out: 10000
```

**important**: `range` in Rust has an *inclusive lower bound and exclusive
upper bound*, meaning that `... (0..100)` is equivalent to `the sum of
... for k=0 to k=n`.

If you're still not absolutely fluent in indexing for these applications, spend some time with [Trev Tutor](https://youtu.be/TDpQSa3hJRw) on youtube.

The notation can be nested, which is much like nesting a `for` loop. You should
evaluate the right-most sigma first, unless the author has enclosed them in
parentheses to alter the order. However, in the following case, since we are
dealing with finite sums, the order does not matter.

![sigma3](http://latex.codecogs.com/svg.latex?%5Csum_%7Bi%3D1%7D%5E%7B2%7D%5Csum_%7Bj%3D4%7D%5E%7B6%7D%283ij%29)

<!-- \sum_{i=1}^{2}\sum_{j=4}^{6}(3ij) -->

In code:

```rust
(1..3i32)
  .map(|i| (4..7i32).map(|j| 3 * i * j).sum::<i32>())
  .sum::<i32>();
// Out: 135
```

## capital Pi

The capital Pi or "Big Pi" is very similar to [Sigma](#sigma), except we are using multiplication to find the product of a sequence of values.

Take the following:

![capitalPi](http://latex.codecogs.com/svg.latex?%5Cprod_%7Bi%3D1%7D%5E%7B6%7Di)

<!-- \prod_{i=1}^{6}i -->

```rust
fn times(x: i64, y: i64) -> i64 {
  x * y
}
```

Or using the function `std::iter::Iterator::fold`

```rust
(1..7).into_iter().fold(1, times);
// # Out: 720
```

## pipes

Pipe symbols, known as *bars*, can mean different things depending on the
context. Below are three common uses: [absolute value](#absolute-value),
[Euclidean norm](#euclidean-norm), and [determinant](#determinant).

These three features all describe the *length* of an object.

### absolute value

![pipes1](http://latex.codecogs.com/svg.latex?%5Cleft%20%7C%20x%20%5Cright%20%7C)

<!-- \left | x \right | -->

For a number *x*, `|x|` means the absolute value of *x*. In code:

```rust
let x = -5
x.abs(x)
// Out: 5
```

### Euclidean norm

![pipes4](http://latex.codecogs.com/svg.latex?%5Cleft%20%5C%7C%20%5Cmathbf%7Bv%7D%20%5Cright%20%5C%7C)

<!-- \left \| \mathbf{v} \right \| -->

For a vector **v**, `โvโ` is the [Euclidean norm](https://en.wikipedia.org/wiki/Norm_%28mathematics%29#Euclidean_norm) of **v**. It is also referred to as the "magnitude" or "length" of a vector.

Often this is represented by double-bars to avoid ambiguity with the *absolute value* notation, but sometimes you may see it with single bars:

![pipes2](http://latex.codecogs.com/svg.latex?%5Cleft%20%7C%20%5Cmathbf%7Bv%7D%20%5Cright%20%7C)

<!-- \left | \mathbf{v} \right | -->

Here is an example using an array `[x, y, z]` to represent a 3D vector.

```rust
let v = vec![0, 4, -3]
v.length
// Out: 5
```

The `length** function:

```rust
fn vec_length(a: Vec<i64>) -> i64 {
    let x = a[0];
    let y = a[1];
    let z = a[2];
    return sqrt(x.pow(2) + y.pow(2) + z.pow(2));
}
```

The implementation for arbitrary length'd vectors is left as an exercise for the
reader.

#### determinant

![pipes3](http://latex.codecogs.com/svg.latex?%5Cleft%20%7C%5Cmathbf%7BA%7D%20%5Cright%20%7C)

<!-- \left |\mathbf{A}  \right | -->

For a matrix **A**, `|A|` means the
[determinant](https://en.wikipedia.org/wiki/Determinant) of matrix **A**.

Here is an example computing the determinant of a 2x2 identity matrix

```rust
let ident_2 = [1., 0., 0., 1.];
let result = nalgebra::Matrix2::from_row_slice(&ident_2);
// Out: 1
```

You should watch [3blue1brown](https://www.youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab), but in short if a matrix (list of list of numbers)
is interpreted as hitting a **coordinate system** with a
*squisher-stretcher-rotater*, the determinant of that matrix is the **measure of
how much the unit area/volume of the coordinate system got
squished-stretched-rotated**.

```rust
// the determinant of the 100 x 100 identity matrix is still one, because the identity matrix doesn't squish, stretch, or rotate at all.
nalgebra::DMatrix::<f32>::identity(100, 100)
// Out: 1.0

// 90 degree rotation.
nalgebra::Matrix2::from_row_slice(&[0., -1., 1., 0.])
// Out: 1.0

```

The second matrix was the [**2D rotation**](https://en.wikipedia.org/wiki/Rotation_matrix) at 90 degrees.

## hat

In geometry, the "hat" symbol above a character is used to represent a [unit vector](https://en.wikipedia.org/wiki/Unit_vector). For example, here is the unit vector of **a**:

![hat](http://latex.codecogs.com/svg.latex?%5Chat%7B%5Cmathbf%7Ba%7D%7D)

<!-- \hat{\mathbf{a}} -->

In Cartesian space, a unit vector is typically length 1. That means each part of the vector will be in the range of -1.0 to 1.0. Here we *normalize* a 3D vector into a unit vector:

```rust
let a = vec![0., 4., -3.];
normalize(a)
// Out: Vec<f64>[0., 0.8, -0.6]
```

If a vector is that which has magnitude and direction, normalization of a vector
is the operation that deletes magnitude and preserves direction.

Here is the `normalize` function, operating on 3D vectors:

```rust
fn normalize(a: Vec<i64>) -> Vec<i64> {
    let mut b = a.to_vec();
    let mut x = a[0];
    let mut y = a[1];
    let mut z = a[2];
    let squared_length = x * x + y * y + z * z;

    if squared_length > 0 {
        let length = sqrt(squared_length);
        b[0] = x / length;
        b[1] = y / length;
        b[2] = z / length;
    }

    return b;
}
```

## element

In set theory, the "element of" symbol `โ` and `โ` can be used to describe whether something is an element of a *set*. For example:

![element1](http://latex.codecogs.com/svg.latex?A%3D%5Cleft%20%5C%7B3%2C9%2C14%7D%7B%20%5Cright%20%5C%7D%2C%203%20%5Cin%20A)

<!-- A=\left \{3,9,14}{  \right \}, 3 \in A -->

Here we have a set of numbers *A* = `{ 3, 9, 14 }` and we are saying `3` is an "element of" that set.

The `in` keyword plays the role of the elementhood function, giving a bool.

```rust
let a = vec![3, 9, 14];
a.contains(&3);
// Out: true
```

Rust also has `set`. You can wrap any iterable `set` keyword to delete
repeats.

```rust
let mut a = vec![3, 3, 3, 2, 4, 3, 3, 3, 1, 2, 4, 5, 3];
let set: std::collections::HashSet<i32> = a.into_iter().collect();
let mut set = set.into_iter().collect::<Vec<i32>>();
&set.sort();
// Out: Vec<i32>[1, 2, 3, 4, 5]

// However the example above works even better with the dedup, which has the same effect of the set
let mut a = vec![3, 3, 3, 2, 4, 3, 3, 3, 1, 2, 4, 5, 3];
a.sort(); // sort
a.dedup(); // remove the duplicated values
// Out: Vec<i32>[1, 2, 3, 4, 5]

let a: Vec<i32> = (1..20).step_by(4).collect();
a.contains(&3);
// Out: false
```

The backwards `โ` is the same, but the order changes:

![element2](http://latex.codecogs.com/svg.latex?A%3D%5Cleft%20%5C%7B3%2C9%2C14%7D%7B%20%5Cright%20%5C%7D%2C%20A%20%5Cni%203)

<!-- A=\left \{3,9,14}{  \right \}, A \ni 3 -->

You can also use the "not an element of" symbols `โ` and `โ` like so:

![element3](http://latex.codecogs.com/svg.latex?A%3D%5Cleft%20%5C%7B3%2C9%2C14%7D%7B%20%5Cright%20%5C%7D%2C%206%20%5Cnotin%20A)

<!-- A=\left \{3,9,14}{  \right \}, 6 \notin A -->

Which you know is represented by the `!` keyword in Rust. For instance negating a value `!(1 == 2)`

## common number sets

You may see some some large [Blackboard](https://en.wikipedia.org/wiki/Blackboard_bold) letters among equations. Often, these are used to describe sets.

For example, we might describe *k* to be an [element of](#element) the set `โ`.

![real](http://latex.codecogs.com/svg.latex?k%20%5Cin%20%5Cmathbb%7BR%7D)

<!-- k \in \mathbb{R} -->

Listed below are a few common sets and their symbols.

### `โ` real numbers

The large `โ` describes the set of *real numbers*. These include integers, as well as rational and irrational numbers.

Computers approximate `โ` with `float`.

You can use `is_finite` to check "*k* โ โ".

```rust
fn is_real<T: num::Float>(x: T) -> bool {
    x.is_finite()
}
// Out: true
```

Again, you may elevate that bool to an `assertion` that makes-or-breaks the whole program
with the `assert` keyword when you see fit.

#### `โ` rational numbers

Rational numbers are real numbers that can be expressed as a fraction, or
*ratio*. Rational numbers cannot have zero as a denominator.

Imagine taking `โ` and removing radicals (like `num::Float::sqrt`) and logarithms (in a
family called
[transcendentals](https://en.wikipedia.org/wiki/Transcendental_function)),
that's basically what `โ` is, at least enough for a rough first approximation.

This also means that all integers are rational numbers, since the denominator can be expressed as 1.

An irrational number, on the other hand, is one that cannot be expressed as a ratio, like ฯ (`std::f64::consts::PI`).

#### `โค` integers

An integer is a whole number. Just imagine starting from zero and one and
building out an inventory with addition and subtraction.

An integer has no division, no decimals.

```rust
let i:i32 = 1;
let j:i64 = 999;
```

#### `โ` natural numbers

A natural number, a non-negative integer.

This is actually the only set invented by the [flying spaghetti monster](https://www.brainyquote.com/quotes/leopold_kronecker_338745): as for the
others, humans have themselves to blame.

Depending on the context and field of study, the set may or may not **start with zero**.

...ok but, between you and me, **they 200% start with zero**.

`โ` also happens to be the first **inductive construction** in the [study of
datatypes](https://en.wikipedia.org/wiki/Semantics_(computer_science)), consisting of a single axiom ("Zero is a `โ`") and a single
inference rule ("if `n` is a `โ` then `n + 1` is also a `โ`")

`โ` in Rust natural numbers are `u8, u16, u32, u64, u128`.

#### `โ` complex numbers

As we saw earlier, the complex numbers are a particular struct.

A complex number is a combination of a real and imaginary number, viewed as a co-ordinate in the 2D plane. For more info, see [A Visual, Intuitive Guide to Imaginary Numbers](http://betterexplained.com/articles/a-visual-intuitive-guide-to-imaginary-numbers/).

We can say `โ = {a + b*i | a,b โ โ}`, which is a notation called

## functions

[Functions](https://en.wikipedia.org/wiki/Function_%28mathematics%29) are fundamental features of mathematics, and the concept is fairly easy to translate into code.

A **function** transforms an input into an output value. For example, the following is a function:

![function1](http://latex.codecogs.com/svg.latex?x%5E%7B2%7D)

<!-- x^{2} -->

We can give this function a *name*. Commonly, we use `ฦ` to describe a function, but it could be named `A` or anything else.

![function2](http://latex.codecogs.com/svg.latex?f%5Cleft%20%28x%20%5Cright%20%29%20%3D%20x%5E%7B2%7D)

<!-- f\left (x  \right ) = x^{2} -->

In code, we might name it `square` and write it like this:

```rust
fn square(x: i64) -> i64 {
    x.pow(2)
}
```

Sometimes a function is not named, and instead the output is written.

![function3](http://latex.codecogs.com/svg.latex?y%20%3D%20x%5E%7B2%7D)

<!-- y = x^{2} -->

In the above example, *x* is the input, the transformation is *squaring*, and *y*
is the output. We can express this as an equation because, conventionally, we
think of *x* as input and *y* as output.

But we have a stronger idea called **anonymous functions** to generalize this.

Just as we can name strings `x = "Alonzo"` then call them with their names *or*
we can just pass string *literals*, we also have **function literals**.

Math first, then Rust:

`x โฆ x^2` is equivalent to the equational description above.

```rust
let lambda_square = |x: i64| x.pow(2);
```

Functions can also have multiple parameters, like in a programming language. These are known as *arguments* in mathematics, and the number of arguments a function takes is known as the *arity* of the function.

![function4](http://latex.codecogs.com/svg.latex?f%28x%2Cy%29%20%3D%20%5Csqrt%7Bx%5E2%20&plus;%20y%5E2%7D)

<!-- f(x,y) = \sqrt{x^2 + y^2} -->

### piecewise function

Some functions will use different relationships depending on the input value, *x*.

The following function *ฦ* chooses between two "sub functions" depending on the input value.

![piecewise1](http://latex.codecogs.com/svg.latex?f%28x%29%3D%20%5Cbegin%7Bcases%7D%20%5Cfrac%7Bx%5E2-x%7D%7Bx%7D%2C%26%20%5Ctext%7Bif%20%7D%20x%5Cgeq%201%5C%5C%200%2C%20%26%20%5Ctext%7Botherwise%7D%20%5Cend%7Bcases%7D)

<!--    f(x)=
\begin{cases}
    \frac{x^2-x}{x},& \text{if } x\geq 1\\
    0, & \text{otherwise}
\end{cases} -->

This is very similar to `if` / `else` in code. The right-side conditions are often written as **"for x < 0"** or **"if x = 0"**. If the condition is true, the function to the left is used.

In piecewise functions, **"otherwise"** and **"elsewhere"** are analogous to the `else` statement in code.

```rust
fn f(x: f64) -> f64 {
    if x >= 1. {
        x.powf(2.) - x / x
    } else {
        0.
    }
}
```

### common functions

There are some function names that are ubiquitous in mathematics. For a programmer, these might be analogous to functions.

One such example is the *sgn* function. This is the *signum* or *sign* function. Let's use [piecewise function](#piecewise-function) notation to describe it:

![sgn](http://latex.codecogs.com/svg.latex?sgn%28x%29%20%3A%3D%20%5Cbegin%7Bcases%7D%20-1%26%20%5Ctext%7Bif%20%7D%20x%20%3C%200%5C%5C%200%2C%20%26%20%5Ctext%7Bif%20%7D%20%7Bx%20%3D%200%7D%5C%5C%201%2C%20%26%20%5Ctext%7Bif%20%7D%20x%20%3E%200%5C%5C%20%5Cend%7Bcases%7D)

<!-- sgn(x) :=
\begin{cases}
    -1& \text{if } x < 0\\
    0, & \text{if } {x = 0}\\
    1, & \text{if } x > 0\\
\end{cases} -->

In code, it might look like this:

```rust
fn sgn(x: i32) -> i32 {
    match x {
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => 0,
    }
}
```

See [signum](https://docs.rs/num/0.1.42/num/fn.signum.html) for this function as a module.

Other examples of such functions: *sin*, *cos*, *tan* can be found on the Rust standard library as traits on [module num](https://docs.rs/num/0.1.42/num/index.html)

### function notation

In some literature, functions may be defined with more explicit notation. For example, let's go back to the `square` function we mentioned earlier:

![function2](http://latex.codecogs.com/svg.latex?f%5Cleft%20%28x%20%5Cright%20%29%20%3D%20x%5E%7B2%7D)

<!-- f\left (x  \right ) = x^{2} -->

It might also be written in the following form:

![mapsto](http://latex.codecogs.com/svg.latex?f%20%3A%20x%20%5Cmapsto%20x%5E2)

<!-- f : x \mapsto x^2 -->

The arrow here with a tail typically means "maps to," as in *x maps to x<sup>2</sup>*.

Sometimes, when it isn't obvious, the notation will also describe the *domain* and *codomain* of the function. A more formal definition of *ฦ* might be written as:

![funcnot](http://latex.codecogs.com/svg.latex?%5Cbegin%7Balign*%7D%20f%20%3A%26%5Cmathbb%7BR%7D%20%5Crightarrow%20%5Cmathbb%7BR%7D%5C%5C%20%26x%20%5Cmapsto%20x%5E2%20%5Cend%7Balign*%7D)

<!-- \begin{align*}
f :&\mathbb{R} \rightarrow \mathbb{R}\\
&x \mapsto x^2
\end{align*}
 -->

A function's *domain* and *codomain* is a bit like its *input* and *output* types, respectively. Here's another example, using our earlier *sgn* function, which outputs an integer:

![domain2](http://latex.codecogs.com/svg.latex?sgn%20%3A%20%5Cmathbb%7BR%7D%20%5Crightarrow%20%5Cmathbb%7BZ%7D)

<!-- sgn : \mathbb{R} \rightarrow \mathbb{Z} -->

The arrow here (without a tail) is used to map one *set* to another.

## prime

The prime symbol (`โฒ`) is often used in variable names to describe things which are similar, without giving it a different name altogether. It can describe the "next value" after some transformation.

For example, if we take a 2D point *(x, y)* and rotate it, you might name the result *(xโฒ, yโฒ)*. Or, the *transpose* of matrix **M** might be named **Mโฒ**.

In code, we typically just assign the variable a more descriptive name, like `transformedPosition`.

For a mathematical [function](#function), the prime symbol often describes the *derivative* of that function. Derivatives will be explained in a future section. Let's take our earlier function:

![function2](http://latex.codecogs.com/svg.latex?f%5Cleft%20%28x%20%5Cright%20%29%20%3D%20x%5E%7B2%7D)

<!-- f\left (x  \right ) = x^{2} -->

Its derivative could be written with a prime `โฒ` symbol:

![prime1](http://latex.codecogs.com/svg.latex?f%27%28x%29%20%3D%202x)

<!-- f'(x) = 2x -->

In code:

```rust
fn f(x: i64) -> i64 {
    x.pow(2)
}

fn f_prime(x: i64) -> i64 {
    return 2 * x;
}
```

Multiple prime symbols can be used to describe the second derivative *ฦโฒโฒ* and third derivative *ฦโฒโฒโฒ*. After this, authors typically express higher orders with roman numerals *ฦ*<sup>IV</sup> or superscript numbers *ฦ*<sup>(n)</sup>.

## floor & ceiling

The special brackets `โxโ` and `โxโ` represent the *floor* and *ceil* functions, respectively.

![floor](http://latex.codecogs.com/svg.latex?floor%28x%29%20%3D%20%5Clfloor%20x%20%5Crfloor)

<!-- floor(x) =  \lfloor x \rfloor -->

![ceil](http://latex.codecogs.com/svg.latex?ceil%28x%29%20%3D%20%5Clceil%20x%20%5Crceil)

<!-- ceil(x) =  \lceil x \rceil -->

In code:

```rust
fn floor<T: num::Float>(x: T) -> T {
    x.floor()
}

fn ceil<T: num::Float>(x: T) -> T {
    x.ceil()
}
```

When the two symbols are mixed `โxโ`, it typically represents a function that rounds to the nearest integer:

![round](http://latex.codecogs.com/svg.latex?round%28x%29%20%3D%20%5Clfloor%20x%20%5Crceil)

<!-- round(x) =  \lfloor x \rceil -->

In code:

```rust
fn round<T: num::Float>(x: T) -> T {
    x.round()
}
```

## arrows

Arrows are often used in [function notation](#function-notation). Here are a few other areas you might see them.

### material implication

Arrows like `โ` and `โ` are sometimes used in logic for *material implication.* That is, if A is true, then B is also true.

![material1](http://latex.codecogs.com/svg.latex?A%20%5CRightarrow%20B)

<!-- A \Rightarrow B -->

Interpreting this as code might look like this:

```rust
if a == true {
  assert!(b == true);
}
```

The arrows can go in either direction `โ` `โ`, or both `โ`. When *A โ B* and *B โ A*, they are said to be equivalent:

![material-equiv](http://latex.codecogs.com/svg.latex?A%20%5CLeftrightarrow%20B)

<!-- A \Leftrightarrow B -->

#### equality

In math, the `<` `>` `โค` and `โฅ` are typically used in the same way we use them in code: *less than*, *greater than*, *less than or equal to* and *greater than or equal to*, respectively.

```rust
50 > 2 == true;
2 < 10 == true;
3 <= 4 == true;
4 >= 4 == true;
```

On rare occasions you might see a slash through these symbols, to describe *not*. As in, *k* is "not greater than" *j*.

![ngt](http://latex.codecogs.com/svg.latex?k%20%5Cngtr%20j)

<!-- k \ngtr j -->

The `โช` and `โซ` are sometimes used to represent *significant* inequality. That is, *k* is an [order of magnitude](https://en.wikipedia.org/wiki/Order_of_magnitude) larger than *j*.

![orderofmag](http://latex.codecogs.com/svg.latex?k%20%5Cgg%20j)

<!-- k \gg j -->

#### conjunction & disjunction

Another use of arrows in logic is conjunction `โง` and disjunction `โจ`. They are analogous to a programmer's `AND` and `OR` operators, respectively.

The following shows conjunction `โง`, the logical `AND`.

![and](http://latex.codecogs.com/svg.latex?k%20%3E%202%20%5Cland%20k%20%3C%204%20%5CLeftrightarrow%20k%20%3D%203)

<!-- k > 2 \land k <  4 \Leftrightarrow k = 3   -->

In Rust, we use `&&`. Assuming *k* is a natural number, the logic implies that *k* is 3:

```rust
if k > 2 && k < 4 {
  assert!(k == 3);
}
```

Since both sides are equivalent `โ`, it also implies the following:

```rust
if k == 3 {
  assert!(k > 2 && k < 4);
}
```

The down arrow `โจ` is logical disjunction, like the OR operator.

![logic-or](http://latex.codecogs.com/svg.latex?A%20%5Clor%20B)

<!-- A \lor B -->

In code:

```rust
A || B
```

## logical negation

Occasionally, the `ยฌ`, `~` and `!` symbols are used to represent logical `NOT`. For example, *ยฌA* is only true if A is false.

Here is a simple example using the *not* symbol:

![negation](http://latex.codecogs.com/svg.latex?x%20%5Cneq%20y%20%5CLeftrightarrow%20%5Clnot%28x%20%3D%20y%29)

<!-- x \neq y \Leftrightarrow \lnot(x = y) -->

An example of how we might interpret this in code:

```rust
if (x != y) {
  assert(!(x == y))
}
```

*Note:* The tilde `~` has many different meanings depending on context. For example, *row equivalence* (matrix theory) or *same order of magnitude* (discussed in [equality](#equality)).

## intervals

Sometimes a function deals with real numbers restricted to some range of values, such a constraint can be represented using an *interval*

For example we can represent the numbers between zero and one including/not including zero and/or one as:

- Not including zero or one: ![interval-opened-left-opened-right](http://latex.codecogs.com/svg.latex?%280%2C%201%29)

<!-- (0, 1) -->

- Including zero or but not one: ![interval-closed-left-opened-right](http://latex.codecogs.com/svg.latex?%5B0%2C%201%29)

<!-- [0, 1) -->

- Not including zero but including one: ![interval-opened-left-closed-right](http://latex.codecogs.com/svg.latex?%280%2C%201%5D)

<!-- (0, 1] -->

- Including zero and one: ![interval-closed-left-closed-right](http://latex.codecogs.com/svg.latex?%5B0%2C%201%5D)

<!-- [0, 1] -->

For example we to indicate that a point `x` is in the unit cube in 3D we say:

![interval-unit-cube](http://latex.codecogs.com/svg.latex?x%20%5Cin%20%5B0%2C%201%5D%5E3)

<!-- x \in [0, 1]^3 -->

In code we can represent an interval using a two element 1d array:

Like this guide? Suggest some [more features](https://github.com/eduardonunesp/math-as-rust/issues/1) or send us a Pull Request!

## Inspiration

This repo is havely inspired, based on the original project by [math-as-code](https://github.com/Jam3/math-as-code), thank you very much to Jam3 for creating that awesome guide which serves as huge inspiration for me to build a Rust version ๐ฆ

## Contributing

For details on how to contribute, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT, see [LICENSE.md](http://github.com/eduardonunesp/math-as-rust/blob/master/LICENSE.md) for details
