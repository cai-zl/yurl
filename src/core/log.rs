#[macro_export]
macro_rules! success {
    ($e:expr) => {
        println!("{}", $e.green())
    };
}

#[macro_export]
macro_rules! error {
    ($e:expr) => {
        println!("{}", $e.red())
    };
}

#[macro_export]
macro_rules! warn {
    ($e:expr) => {
        println!("{}", $e.yellow())
    };
}
