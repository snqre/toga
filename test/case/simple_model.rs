trait Health<T> {

    fn health(&self) -> T;
}

trait Wizard {

    fn you_shall_not_pass(&self) {}
}

struct Player<const A: usize, B>(B); 

toga::blockset! { 
    impl<const A: usize, B> Player<A, B>
    where
        B: Clone;

    pub fn hello_world(&self) {}

    pub fn give_me_a_number(&self) -> u8 {
        50
    }

    self::Wizard {}

    self::Health<u8> {
        fn health(&self) -> u8 {
            100
        }
    }
}

fn main() {
    let player: Player<5, _> = Player("Hello World");
    assert_eq!(player.health(), 100);
    assert_eq!(player.give_me_a_number(), 50);
    player.you_shall_not_pass();
    player.hello_world();
}