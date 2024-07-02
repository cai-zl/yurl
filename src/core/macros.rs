#[macro_export]
macro_rules! println_green {
    ($e:expr) => {
        println!("{}", $e.green())
    };
}

#[macro_export]
macro_rules! println_red {
    ($e:expr) => {
        println!("{}", $e.red())
    };
}

#[macro_export]
macro_rules! println_yellow {
    ($e:expr) => {
        println!("{}", $e.yellow())
    };
}

#[macro_export]
macro_rules! yurl_error {
    ($e:expr) => {
        Box::new(YurlError::new($e))
    };
}
