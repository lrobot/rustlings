// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    
    let a:[usize;100] =  core::array::from_fn(|i|i+1);

    if a.len() >= 100 {
        println!("Wow, that's a big array!{:#?}",a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.{}", a.len());
    }
}
