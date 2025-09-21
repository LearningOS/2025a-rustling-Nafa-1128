// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut left = 0;
    let mut right = input.len();
    for i in input.chars(){
        if i != ' '{
            break;
        }
        left += 1;
    }
    for i in input.chars().rev(){
        if i != ' '{
            break;
        }
        right -= 1;
    }
    input[left..right].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s = String::from(input);
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut s = String::from("");
    for word in input.split_whitespace(){
        if word == "cars"{
            s.push_str("balloons");
        }else{
            s.push_str(word);
        }
        s.push(' ');
    }
    s.pop();
    s
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
