// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // ???
    // let input1 = input.to_string();
    let mut str1 = String::new();
    let mut meetChar = false;
    for ch in input.chars().rev() {
        if ch != ' ' {
            meetChar = true;
            str1.push(ch)
        } else {
            if meetChar {
                str1.push(ch);
            }
        }
    }
    meetChar = false;
    let mut str2 = String::new();
    for ch in str1.chars().rev() {
        if ch != ' ' {
            meetChar = true;
            str2.push(ch)
        } else {
            if meetChar {
                str2.push(ch);
            }
        }
    }
    str2
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    // ???
    let mut str1 = input.to_string();
    str1.push_str(" world!");
    str1
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // ???
    let a = input.replace("cars", "balloons");
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
