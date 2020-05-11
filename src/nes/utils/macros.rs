#[macro_export]
macro_rules! multiline_println {
    ($($s:literal),+) => {
        $(
        println!("{}", $s);
        )*
    }
}
