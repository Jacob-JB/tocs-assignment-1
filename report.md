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
