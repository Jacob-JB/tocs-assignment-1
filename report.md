# Style of parser

My parser attempts to apply productions recursively to determine if the input string conforms to the language.
Each production is associated with a function that takes an input string and returns a success or fail state.
If the function is successful it returns a subslice of the input string with the tokens that belong to the production removed.

The language being LL(1) means that these production functions only ever need to inspect the next token to determine if the input string is correct.
As soon as a token that doesn't match the production is encountered, the function returns a fail state.

This means that where a production expects a set of specific characters it can check for them directly,
or if it checks it expects another production function to match it can call them and propogate fail states up the call stack.

Each production function has this signature

```rs
fn production_function(input: RemainingInput) -> Result<RemainingInput, ()>
```

The function returns `Ok(RemainingInput)` if the production matches the input, or `Err(())` if it does not.
The returned `RemainingInput` is the remaining input after the production.

The type `RemainingInput` is a newtype over a `&str`, which is a fat pointer to an array UTF-8 bytes. This means that copying and passing around `RemainingInput` values is cheap and allows the parser to never make any heap allocations.

# How would my parser accomodate changes?

Any changes to the language must keep it LL(1).
More productions would require adding additional production functions and test cases.

# Test coverage

Each production function is given a single test case, each test case contains multiple assertions.
In each test case there are tests that cover multipl cases where the production function should succeed *and* fail.

Breaking up each production into its own function allows me to test each one individually,
allowing the test cases to remain simple for the less complex productions,
and use test more complex input strings for the more complex non terminals.

# Language choice

### Pros:

Rust's string slices (&str) and lifetimes let me handle pointers to strings without having to worry about memory bugs.
Additionally the entire parser makes no heap allocations.

Pattern matching over strings makes checking for sets of characters easy.

The rust std library has built in support for UTF 8 correctness which provides robustness when dealing with multi byte characters such as ×, ∸, ⊤ etc.

This code snippet highlights some of these points.
```rs
fn production_digit(input: RemainingInput) -> Result<RemainingInput, ()> {
    match input.peek()? {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Ok(input.slice()),
        _ => Err(()),
    }
}
```

`input.peek()` returns a `char` which is a valide unicode code point, I don't have to worry about multi byte characters.

`input.slice()` returns a subslice with the first character removed, no copying or allocations.

The match statement makes checking for valid digits concise and readable, allows compiler to optimize the best comparison logic.

The residual operator `?` shows one of the reasons I designed my function signatures around `Result`, input.peek returns an `Err` if the input is empty and I can early return very concisely. It allows me to propagate errors with less boilerplate.

### Cons:

Working with lifetimes and references instead of pointers can be more complex. For this assignment it was a simple case but it can get especially more complex, especially if for example i wanted to use multiple cores working on the same data.

All the niceties of the ownership model/borrow checking and others increase compile times.

### Outcome:

Rust is a great choice for a task like this if the speed of the parser is important.
If the benefits of zero copies are not important there are other languages that could produce better ergonomics in some areas.
This would especially be the case for creating an AST, where in rust I would need to work with safe pointer types such as `Box` or `Rc`/`Arc`, in a language such as python or java, references to objects are handled easily.
