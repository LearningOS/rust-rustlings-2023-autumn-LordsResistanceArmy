// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



// 方法三
fn main() {
    macros::my_macro!();
}

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}

// //方法二
// fn main() {
//     my_macro!();
// }

// mod macros {
//     #[macro_export]
//     macro_rules! my_macro {
//         () => {
//             println!("Check out my macro!");
//         };
//     }
// }

// // 方法一
// #[macro_use]
// mod macros {
//     macro_rules! my_macro {
//         () => {
//             println!("Check out my macro!");
//         };
//     }
// }

// fn main() {
//     my_macro!();
// }