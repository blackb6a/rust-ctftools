#[macro_export]
macro_rules! main { 
    ($($t: tt)*) => {
        fn main() {
            $($t)*
        }
    };
}