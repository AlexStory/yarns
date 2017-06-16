pub fn head(word: &String) -> String {
    word.chars().take(1).collect()
}

pub fn substr(word: &String, start:usize, length:usize) -> String {
    let res: String = word.chars().skip(start).take(length).collect();
    res
}

pub fn concat(str1: &String, str2: &String) -> String {
    format!("{}{}", str1, str2)
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
}
