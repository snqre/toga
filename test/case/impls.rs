trait Health<T> {
    fn health(&self) -> T;
}

trait Wizard {
    fn you_shall_not_pass(&self) {}
}

struct Player<const A: usize, B>(B); 

toga::impls!(
    impl<const A: usize, B> Player<A, B>
    where
        B: Clone;

    #[inline]
    pub fn say_hello(&self, num: u8) {
        
    }

    #[inline]
    pub fn hello_world(&self) {}
    pub fn give_me_a_number(&self) -> u8 {
        50
    }

    impl Wizard {
        #[inline]
        fn you_shall_not_pass(&self) {}
    }
    
    impl Health<u8> {
        fn health(&self) -> u8 {
            100
        }
    }
);

fn main() {
    let player: Player<5, _> = Player("Hello World");
    assert_eq!(player.health(), 100);
    assert_eq!(player.give_me_a_number(), 50);
    player.you_shall_not_pass();
    player.hello_world();
    player.say_hello(10);
}