# yarns

A simple (beginner's) library for working with strings in rust. Built for ease, not speed.

## head
(&String) -> String
Borrows a string and returns the first character of it as a string.

## substr
(&String, usize, usize) -> String
Borrows a string and two usizes, and gives you the substring, from the index of the first number, with the length of the second.

## concat
(&String, &String) -> String
Borrows two strings and concatenates them.