fn fizz_buzz1() { 
    let mut x = 1;

    while x <= 100 {
        print!(" ");
        if x % 15 == 0 {
            print!("FizzBuzz");
        } else if x % 3 == 0 {
            print!("Fizz");
        } else if x % 5 == 0 {
            print!("Buzz");
        } else {
            print!("{}", x);
        }

    x += 1;
    }
}

fn main() {
    fizz_buzz1();
}
