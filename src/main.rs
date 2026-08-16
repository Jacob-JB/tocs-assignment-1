use std::process::ExitCode;

fn main() -> ExitCode {
    let mut input = String::new();

    let Ok(_) = std::io::stdin().read_line(&mut input) else {
        return ExitCode::FAILURE;
    };

    let input = input.trim_matches(&['\n', '\r']);

    println!("Input: \"{}\"", input);

    match production_input(RemainingInput(&input)) {
        // remaining input must be completely consumed.
        Ok(input) if input.peek().is_err() => {
            println!("Valid");

            ExitCode::SUCCESS
        }
        _ => {
            println!("Invalid");

            ExitCode::FAILURE
        }
    }
}

/// Stores a string slice to the remaining input of the parser.
///
/// Can be used to peek at the next character and take a slice of the following characters after the first.\
#[derive(Clone, Copy)]
struct RemainingInput<'a>(&'a str);

impl<'a> RemainingInput<'a> {
    /// Attempts to peek at the next character in the input.
    ///
    /// Returns `Ok` with the character if it exists, or `Err` if there is no input remaining.
    pub fn peek(&self) -> Result<char, ()> {
        self.0.chars().next().ok_or(())
    }

    /// Returns a new `RemainingInput` with the first character removed.
    ///
    /// Panics if the input is empty.
    pub fn slice(self) -> RemainingInput<'a> {
        let next_boundary = self
            .0
            .char_indices()
            .nth(1)
            .map_or(self.0.len(), |(i, _)| i);

        RemainingInput(&self.0[next_boundary..])
    }
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `input = { " " } , expression , { " " } ;`
fn production_input(input: RemainingInput) -> Result<RemainingInput, ()> {
    let input = production_whitespace(input);

    let input = production_expression(input)?;

    let input = production_whitespace(input);

    Ok(input)
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `expression = numeral | variable | glyph | abstraction | application ;`
fn production_expression(input: RemainingInput) -> Result<RemainingInput, ()> {
    if let Ok(input) = production_numeral(input) {
        return Ok(input);
    }

    if let Ok(input) = production_variable(input) {
        return Ok(input);
    }

    if let Ok(input) = production_glyph(input) {
        return Ok(input);
    }

    if let Ok(input) = production_abstraction(input) {
        return Ok(input);
    }

    if let Ok(input) = production_application(input) {
        return Ok(input);
    }

    Err(())
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `numeral = digit , { digit } ;`
fn production_numeral(input: RemainingInput) -> Result<RemainingInput, ()> {
    // must match at least one digit.
    let Ok(mut input) = production_digit(input) else {
        return Err(());
    };

    // then zero or more digits.
    loop {
        if let Ok(i) = production_digit(input) {
            input = i;
            continue;
        }

        break;
    }

    Ok(input)
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;`
fn production_digit(input: RemainingInput) -> Result<RemainingInput, ()> {
    match input.peek()? {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Ok(input.slice()),
        _ => Err(()),
    }
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `variable = letter , { letter | digit | "-" } ;`
fn production_variable(input: RemainingInput) -> Result<RemainingInput, ()> {
    // must match at least one letter.
    let Ok(mut input) = production_letter(input) else {
        return Err(());
    };

    // Then zero or more of any of these.
    loop {
        if let Ok(i) = production_letter(input) {
            input = i;
            continue;
        }

        if let Ok(i) = production_digit(input) {
            input = i;
            continue;
        }

        if let Ok('-') = input.peek() {
            input = input.slice();
            continue;
        }

        break;
    }

    Ok(input)
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `
/// letter = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"
///        | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t"
///        | "u" | "v" | "w" | "x" | "y" | "z" ;
/// `
fn production_letter(input: RemainingInput) -> Result<RemainingInput, ()> {
    match input.peek()? {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o'
        | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => Ok(input.slice()),
        _ => Err(()),
    }
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `glyph = "+" | "×" | "∸" | "⊤" | "⊥" | "←" | "↑" | "→" | "↓" | "∷" | "Θ" ;`
fn production_glyph(input: RemainingInput) -> Result<RemainingInput, ()> {
    match input.peek()? {
        '+' | '×' | '∸' | '⊤' | '⊥' | '←' | '↑' | '→' | '↓' | '∷' | 'Θ' => {
            Ok(input.slice())
        }
        _ => Err(()),
    }
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `abstraction = "λ" , { " " } , variable , { " " } , "." ,  { " " } ,expression ;`
fn production_abstraction(input: RemainingInput) -> Result<RemainingInput, ()> {
    // Lambda
    let 'λ' = input.peek()? else {
        return Err(());
    };
    let input = input.slice();

    // Whitespace
    let input = production_whitespace(input);

    // Variable
    let input = production_variable(input)?;

    // Whitespace
    let input = production_whitespace(input);

    // Period
    let '.' = input.peek()? else {
        return Err(());
    };
    let input = input.slice();

    // Whitespace
    let input = production_whitespace(input);

    // Expression
    let input = production_expression(input)?;

    Ok(input)
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Returns `Err` if the input doesn't conform to the production's grammar.
///
/// `application = "(" , { " " } , expression , " ", { " " } , expression , { " " } , ")" ;`
fn production_application(input: RemainingInput) -> Result<RemainingInput, ()> {
    // Opening parenthesis
    let '(' = input.peek()? else {
        return Err(());
    };
    let input = input.slice();

    // Whitespace
    let input = production_whitespace(input);

    // First expression
    let input = production_expression(input)?;

    // At least one space
    let ' ' = input.peek()? else {
        return Err(());
    };
    let input = input.slice();

    // Whitespace
    let input = production_whitespace(input);

    // Second expression
    let input = production_expression(input)?;

    // Whitespace
    let input = production_whitespace(input);

    // Closing parenthesis
    let ')' = input.peek()? else {
        return Err(());
    };
    let input = input.slice();

    Ok(input)
}

/// Returns `Ok` with whatever input wasn't consumed by the production.
///
/// Cannot return `Err` as the empty string is a valid input.
///
/// Not in the specification but serves as a convenience for whitespace handling.
///
/// `whitespace = { " " } ;`
fn production_whitespace(mut input: RemainingInput) -> RemainingInput {
    loop {
        let Ok(' ') = input.peek() else {
            break;
        };

        input = input.slice();
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        // Some complex inputs with and without whitespace front and back.

        assert!(matches!(
            production_input(RemainingInput(" (λa.(λa.1  ×) +) ")),
            Ok(RemainingInput(""))
        ));

        assert!(matches!(
            production_input(RemainingInput("(λa.abc- 1)   ")),
            Ok(RemainingInput(""))
        ));

        assert!(matches!(
            production_input(RemainingInput("   (+ (λa.(λa.1 ab-1) 1))")),
            Ok(RemainingInput(""))
        ));
    }

    #[test]
    fn expression() {
        // Accepts every type of expression

        // numeral
        assert!(matches!(
            production_expression(RemainingInput("123")),
            Ok(RemainingInput(""))
        ));

        // variable
        assert!(matches!(
            production_expression(RemainingInput("abc123-")),
            Ok(RemainingInput(""))
        ));

        // glyph
        assert!(matches!(
            production_expression(RemainingInput("+")),
            Ok(RemainingInput(""))
        ));

        // abstraction
        assert!(matches!(
            production_expression(RemainingInput("λa.1")),
            Ok(RemainingInput(""))
        ));

        // application
        assert!(matches!(
            production_expression(RemainingInput("(1 1)")),
            Ok(RemainingInput(""))
        ));

        // Must be one of the above
        assert!(matches!(
            production_expression(RemainingInput(")")),
            Err(())
        ));
    }

    #[test]
    fn numeral() {
        // Takes all digits
        assert!(matches!(
            production_numeral(RemainingInput("1234abc")),
            Ok(RemainingInput("abc"))
        ));

        // Takes one digit
        assert!(matches!(
            production_numeral(RemainingInput("1abc")),
            Ok(RemainingInput("abc"))
        ));

        // Fails if not at least one digit
        assert!(matches!(production_numeral(RemainingInput("abc")), Err(())));
    }

    #[test]
    fn digit() {
        // Accepts all digits
        for c in "0123456789".chars() {
            assert!(matches!(
                production_digit(RemainingInput(&String::from(c))),
                Ok(RemainingInput(""))
            ));
        }

        // Takes only one digit
        assert!(matches!(
            production_digit(RemainingInput("1abc")),
            Ok(RemainingInput("abc"))
        ));

        // Fails if no digits
        assert!(matches!(production_digit(RemainingInput("abc")), Err(())));
    }

    #[test]
    fn variable() {
        // Accepts a valid variable
        assert!(matches!(
            production_variable(RemainingInput("ab12-3c- 123")),
            Ok(RemainingInput(" 123"))
        ));

        // Digit cannot come first
        assert!(matches!(
            production_variable(RemainingInput("123abc")),
            Err(())
        ));

        // Dash cannot come first
        assert!(matches!(
            production_variable(RemainingInput("-123")),
            Err(())
        ));
    }

    #[test]
    fn letter() {
        // Accepts all letters
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            assert!(matches!(
                production_letter(RemainingInput(&String::from(c))),
                Ok(RemainingInput(""))
            ));
        }

        // Takes only one letter
        assert!(matches!(
            production_letter(RemainingInput("abc123")),
            Ok(RemainingInput("bc123"))
        ));

        // Fails if no letter first
        assert!(matches!(
            production_letter(RemainingInput("123abc")),
            Err(())
        ));
    }

    #[test]
    fn glyph() {
        // Accepts all glyps
        for c in "+×∸⊤⊥←↑→↓∷Θ".chars() {
            assert!(matches!(
                production_glyph(RemainingInput(&String::from(c))),
                Ok(RemainingInput(""))
            ));
        }

        // Takes only one glyph
        assert!(matches!(
            production_glyph(RemainingInput("+abc123")),
            Ok(RemainingInput("abc123"))
        ));

        // Fails if no glyph first
        assert!(matches!(production_glyph(RemainingInput("123+")), Err(())));
    }

    #[test]
    fn abstraction() {
        // Accepts a valid abstraction
        assert!(matches!(
            production_abstraction(RemainingInput("λ   a123-bc.  (1 2345)abc")),
            Ok(RemainingInput("abc"))
        ));

        // Lambda must be first
        assert!(matches!(
            production_abstraction(RemainingInput("123abc")),
            Err(())
        ));

        // Fails on first expression
        assert!(matches!(
            production_abstraction(RemainingInput("λ)")),
            Err(())
        ));

        // Fails on period
        assert!(matches!(
            production_abstraction(RemainingInput("λa)")),
            Err(())
        ));

        // Fails on second expression
        assert!(matches!(
            production_abstraction(RemainingInput("λa.)")),
            Err(())
        ));
    }

    #[test]
    fn application() {
        // Accepts a valid application
        assert!(matches!(
            production_application(RemainingInput("(  abc123    λa.1 )abc")),
            Ok(RemainingInput("abc"))
        ));

        // Fails with no whitespace
        assert!(matches!(
            production_application(RemainingInput("(123abc)")),
            Err(())
        ));
    }
}
