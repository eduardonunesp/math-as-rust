# math-as-rust

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
- [equals `=` `≈` `≠` `:=`](#equals-symbols)
- [square root and complex numbers `√` *`i`*](#square-root-and-complex-numbers)
- [dot & cross `·` `×` `∘`](#dot--cross)
  - [scalar multiplication](#scalar-multiplication)
  - [vector multiplication](#vector-multiplication)
  - [dot product](#dot-product)
  - [cross product](#cross-product)
- [sigma `Σ`](#sigma) - *summation*
- [capital Pi `Π`](#capital-pi) - *products of sequences*
- [pipes `||`](#pipes)
  - [absolute value](#absolute-value)
  - [Euclidean norm](#euclidean-norm)
  - [determinant](#determinant)
- [hat **`â`**](#hat) - *unit vector*
- ["element of" `∈` `∉`](#element)
- [common number sets `ℝ` `ℤ` `ℚ` `ℕ`](#common-number-sets)
- [function `ƒ`](#function)
  - [piecewise function](#piecewise-function)
  - [common functions](#common-functions)
  - [function notation `↦` `→`](#function-notation)
- [prime `′`](#prime)
- [floor & ceiling `⌊` `⌉`](#floor--ceiling)
- [arrows](#arrows)
  - [material implication `⇒` `→`](#material-implication)
  - [equality `<` `≥` `≫`](#equality)
  - [conjunction & disjunction `∧` `∨`](#conjunction--disjunction)
- [logical negation `¬` `~` `!`](#logical-negation)
- [intervals](#intervals)
- [more...](#more)

## variable name conventions

There are a variety of naming conventions depending on the context and field of study, and they are not always consistent. However, in some of the literature you may find variable names to follow a pattern like so:

- *s* - italic lowercase letters for scalars (e.g. a number)
- **x** - bold lowercase letters for vectors (e.g. a 2D point)
- **A** - bold uppercase letters for matrices (e.g. a 3D transformation)
- *θ* - italic lowercase Greek letters for constants and special variables (e.g. [polar angle *θ*, *theta*](https://en.wikipedia.org/wiki/Spherical_coordinate_system))

This will also be the format of this guide.

### Rust Math Lib?

## equals symbols

There are a number of symbols resembling the equals sign `=`. Here are a few common examples:

- `=` is for equality (values are the same)
- `≠` is for inequality (value are not the same)
- `≈` is for approximately equal to (`π ≈ 3.14159`)
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

The dot `·` and cross `×` symbols have different uses depending on context.

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

To denote multiplication of one vector with a scalar, or element-wise multiplication of a vector with another vector, we typically do not use the dot `·` or cross `×` symbols. These have different meanings in linear algebra, discussed shortly.

Let's take our earlier example but apply it to vectors. For element-wise vector multiplication, you might see an open dot `∘` to represent the [Hadamard product](https://en.wikipedia.org/wiki/Hadamard_product_%28matrices%29).<sup>[2]</sup>

![dotcross3](http://latex.codecogs.com/svg.latex?3%5Cmathbf%7Bk%7D%5Ccirc%5Cmathbf%7Bj%7D)

<!-- 3\mathbf{k}\circ\mathbf{j} -->

In other instances, the author might explicitly define a different notation, such as a circled dot `⊙` or a filled circle `●`.<sup>[3]</sup>

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
