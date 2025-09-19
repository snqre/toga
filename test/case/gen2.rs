

::toga::toga!(

    pub struct Person
        .T
        .T@Debug
        .T@Clone
        .T@Copy {
        
    }

    pub struct<T> Person 
    where
        T: Debug,
        T: Clone,
        T: Copy {

    }

    pub
    where T
    where T as Debug
    where T as Clone
    where T as Copy {
        name: T,
        age: u8
    }

    pub fn do_something()
        where T
        where T as Debug
        where T as Clone
        where T as Copy
        where () as Supported<T> {
        
    }

    #[where(T(Debug))]
    #[where(T(Clone))]
    pub fn do_something(something: T): T {

    }

    impl ops::Add for Person 
    where impl T
    where impl T as Debug 
    where impl T as Clone
    where impl T as Copy
    where type T {

    }

);