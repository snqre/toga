use toga::injectable;
use toga;

#[injectable]
#[inject(Age(
    + Debug
    + Sized
    + Clone
    + Copy))]
fn guess_age(item: Age) {
    println!("{:?}", item);
}

fn main() {
    guess_age::<u16>(500);
}