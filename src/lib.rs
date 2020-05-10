#![feature(track_caller)]
pub mod style;
pub use style::{Style, *};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
