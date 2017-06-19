use std::io::stdin;

/// Gets the first char of a string, and returns it as string.
///
/// ```
/// let hello = String::from("Hello");
/// let first = yarns::head(&hello);
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
/// let hw = yarns::concat(&hello, &world);
/// //hw = (String)"HelloWorld"
/// ```

pub fn concat(str1: &String, str2: &String) -> String {
    format!("{}{}", str1, str2)
}

/// Returns wether a given String contains the given Substring.
///
/// ```
/// let hello = String::from("Hello");
/// let ell = String::from("ell");
/// let has_it = yarns::contains(&hello, &ell);
/// //has_it = (bool) true
/// ```

pub fn contains(main: &String, sub: &String) -> bool {
    main[..].contains(&sub[..])
}

/// Shortcut for simple binding of user input to an immutable String.
pub fn read() -> String {
    let mut temp = String::new();
    stdin().read_line(& mut temp).expect("failed to read user input");
    temp.trim().to_string()
}

///  Returns the length of the utf-8 representation of a string.
///
/// ```
/// let hello = String::from("Hello");
/// let length = yarns::len(&hello);
/// //length = (i32) 5
/// ```

pub fn len(string: &String) -> i32 {
    let mut count = 0;
    for c in string.chars() {
        count = count + 1;
    }
    count
}
