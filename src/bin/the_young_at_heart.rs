
fn main() {
    let mut age = 0;

    for n in 3..1000 {
        let divisible_by_3 = n % 3 == 0;
        let divisible_by_5 = n % 5 == 0;

        if divisible_by_3 || divisible_by_5 {
            age += n;
        }
    }

    println!("The wizard's age is {}!", age);
}
