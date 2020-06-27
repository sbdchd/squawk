pub mod ast;
pub mod error;
pub mod parse;

pub fn foo() {
    println!("foobar");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
