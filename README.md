# NLP_Math_Engine

A Rust-based natural language mathematics engine that interprets and executes mathematical operations expressed in plain English.

The goal of this project is to explore how far traditional parsing, grammar rules, and symbolic computation can go in solving text-based mathematics problems without relying on neural networks or large language models.

While modern AI systems can solve these tasks with ease, this project focuses on understanding the underlying mechanics of language interpretation, expression parsing, and mathematical reasoning from first principles.

---

## Features

### Basic Mathematical Statements

* Natural language addition

  * `Add 5 and six`
  * `Add five and six`
* Natural language subtraction

  * `Subtract 40 from 20`
* Nested mathematical expressions

  * `Multiply 4 with result of 7 * 7`
* Exponentiation with correct right associativity

  * `2 ^ 3 ^ 4`

* Parentheses support
* Comparison operators

  * Greater than (`>`)
  * Less than (`<`)
  * Greater than or equal (`>=`)
  * Less than or equal (`<=`)
  * Equality (`==`)
---
### Conditional Statements

* Conditional execution using `if`
* Conditional execution using `unless`
* Support for complex chained expressions

Examples:

```text
Add six to the result of subtracting ten from twenty, then multiply by three unless the result is negative
```

```text
Add six to the result of subtracting thirty from twenty, then multiply by three unless the result is negative
```

```text
Add sixty seven to the result of subtracting hundred from thousand, then multiply it by three if the result is greater than or equal to 900
```
---
### Variable Finding
* Solving for variables in simple equations
Examples:
```text
solve x in x = 5! / 2 
```
```text
What is the value of x if x + 5 = 10
```
```text
Solve x in (((((x + 1) + 2) + 3) + 4) + 5) = 20
```

```text 
Solve x in cos(x) = 0.5"
```

```text
Solve x in sin(x + 30) = 0.5
```

---
### Trigonometric Functions

Supported functions:

```text
sine of 30

cosine of 60

tangent of 45
```


Identity evaluation:

```text
sin 30 ^ 2 + cos 30 ^ 2
```


Inverse trigonometric functions:

```text
inverse sine of 0.5

inverse cosine of 0.5

inverse tangent of 1
```



Supported units:

```text
degrees

radians
```


---

## Current Status

### Completed

* Expression parser
* Arithmetic operations
* Parentheses handling
* Exponentiation
* Comparison expressions
* Conditional statements (`if`, `unless`)
* Word-to-number conversion
* Nested expression evaluation
* Trigonometric functions

### In Progress

* Trigonometric identities
* Additional grammar patterns
* Performance optimizations
* Expanded test coverage

---

## Why This Project?

This project exists primarily as an experiment in symbolic reasoning and natural language parsing.

Rather than asking *"How can an AI solve this?"*, the question is:

> "How much of natural mathematical language can be understood and executed using only deterministic parsing and computation before we need neural networks?"

The answer is probably "more than expected."

---
