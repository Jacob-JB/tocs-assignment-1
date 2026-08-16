LL(1) means that the parser can figure out exactly which production to use next based on only the next token.
The design pattern I have used is for each production to be given its own function that either succesfully returns with a truncated input string, or fails as soon as the input no longer matches the production.

Each production function has this signature

```rs
fn production_function(input: RemainingInput) -> Result<RemainingInput, ()>
```

The function returns `Ok(RemainingInput)` if the production matches the input, or `Err(())` if it does not.
The returned `RemainingInput` is the remaining input after the production.

The type `RemainingInput` is a newtype over a `&str`, which is a fat pointer to an array UTF-8 bytes. This means that copying and passing around `RemainingInput` values is cheap and allows the parser to never make any heap allocations.
