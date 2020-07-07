// pub struct Guess {
//     value: u32,
// }

// impl Guess {
//     pub fn new(value: u32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2))
//     }

//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3))
//     }

//     #[test]
//     fn one_hundred() {
//         assert_eq!(102, add_two(100))
//     }
// }

// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4)
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // unimplemented!();
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4)
//     }
// }

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4)
}

#[test]
#[ignore]
fn expensive_test() {
    // unimplemented!();
}
