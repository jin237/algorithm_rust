fn fizz_buzz2() {
    for x in 1 .. 101 {
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
    }
}

fn main(){
    fizz_buzz2();
}