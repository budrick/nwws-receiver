// use nom::{
//     character::complete::{char, digit1},
//     combinator::map_res,
//     sequence::{delimited, separated_pair},
//     IResult,
// };
// use std::error::Error;
mod action;
mod class;
mod phenomena;

struct PVtec {
    class: class::PVtecClass,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
