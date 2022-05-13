pub fn add_two(a: i32) -> i32 {
    a + 3
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() -> Result<(), String> {
        let x = foo()?
        if x == 0 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
