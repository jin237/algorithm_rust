fn fizz_buzz4() {
    for x in 1 ..= 100 {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"), // 変数パターンとパターンガードを用いたパターン
            e if e % 3 == 0 => println!("Fizz"),
            e if e % 5 == 0 => println!("Buzz"),
            e => println!("{}", e),
        }
    }
}

fn main() {
    fizz_buzz4();
}
