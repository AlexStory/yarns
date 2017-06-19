extern crate yarns;

mod main {
    use yarns::*;
    
    #[test]
    fn head_hello_h() {
        assert_eq!("h", head(&"hello".to_string()));
    }

    #[test]
    fn substr_hello_1_3() {
        let val = String::from("Hello");
        assert_eq!("ell", substr(&val, 1, 3));
    }

    #[test]
    fn concat_hello_goodbye() {
        let val1 = String::from("Hello");
        let val2 = String::from("Goodbye");
        assert_eq!("HelloGoodbye", concat(&val1, &val2));
    }

    #[test]
    fn contains_hello_hell() {
        let val1 = String::from("hello");
        let val2 = String::from("hell");
        assert_eq!(true, contains(&val1, &val2));
    }

    #[test]
    fn len_hello() {
        let val = String::from("Hello");
        assert_eq!(5, len(&val));
    }
}
