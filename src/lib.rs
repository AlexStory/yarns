/// Gets the first char of a string, and returns it as string.
///
/// ```
/// let hello = String::from("Hello");
/// let first = yarns::head(hello);
/// //first = (String)"H"
/// ```

pub fn head(word: &String) -> String {
    word.chars().take(1).collect()
}

/// Returns a new string starting at the index provided in the second argument, going the length provided as the third argument.
///
/// ```
/// let hello = String::from("Hello");
/// let sub= yarns::substr(&hello, 1, 3);
/// //sub = (String) "ell"
/// ```

pub fn substr(word: &String, start:usize, length:usize) -> String {
    let res: String = word.chars().skip(start).take(length).collect();
    res
}

/// Concatenates two strings.
///
/// ```
/// let hello = String::from("Hello");
/// let world = String::from("World");
/// let hw = yarns.concat(&hello, &world);
/// //hw = (String)"HelloWorld"

pub fn concat(str1: &String, str2: &String) -> String {
    format!("{}{}", str1, str2)
}

/// Returns wether a given String contains the given Substring.
///
/// ```
/// let hello = String::from("Hello");
/// let ell = String::from("ell");
/// let has_it = yarns.contains(&hello, &ell);
/// //has_it = (bool) true

pub fn contains(main: &String, sub: &String) -> bool {
    main[..].contains(&sub[..])
}

#[cfg(test)]
mod tests {
    #[test]
    fn head_hello_h() {
        assert_eq!("h", ::head(&"hello".to_string()));
    }

    #[test]
    fn substr_hello_1_3() {
        let val = String::from("Hello");
        assert_eq!("ell", ::substr(&val, 1, 3));
    }

    #[test]
    fn concat_hello_goodbye() {
        let val1 = String::from("Hello");
        let val2 = String::from("Goodbye");
        assert_eq!("HelloGoodbye", ::concat(&val1, &val2));
    }

    #[test]
    fn contains_hello_hell() {
        let val1 = String::from("hello");
        let val2 = String::from("hell");
        assert_eq!(true, ::contains(&val1, &val2));
    }
}
