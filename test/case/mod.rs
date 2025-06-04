

#[model]
mod hello_world {
    use core::ops;



    // automatically generate
    // pub type Result<Ok> = ::core::result::Result<Ok, Error>;
    #[error]
    enum Error {}
    
    #[main]
    struct HelloWorld {}

    #[static]
    say_hello() {
        println!("Hello")
    }

    #[method]
    hello() -> u8 {
        200
    }


    #[ops::Add]
    type ..

    fn add() {
        ..
    }
}





fn do_something<Name>(name: Name) -> u8
where
    Name: self::Name {
    name.timestamp()
}