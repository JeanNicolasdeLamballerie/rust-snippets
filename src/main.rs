use std::{
    cmp::Ordering,
    io::{self},
};
// standard std exposing io methods
use rand::Rng;
// Rng above is a Trait > exposes the required methods

fn main() {
    loop {
        let mut command = String::new();
        println!("enter [e]xit to quit...");
        println!("Please choose the rust playground to launch : ");
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line...");

        match command.trim() {
            "guess" | "g" => guessing_game(),
            "ifelse" | "i" => if_else(),
            "scoped" => scoped_reassignment(),
            "structs" | "s" => testing_structs(),
            "exit" | "e" => break,
            _ => continue,
        }
    }
}

fn guessing_game() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // mut > mutable
    let mut attempts = 0u32;
    loop {
        attempts += 1;
        println!("Attempt n°{attempts}...");
        println!("Please input your guess : ");

        let mut guess = String::new();
        io::stdin()
            // &mut => reference that's mutable (& = reference)
            .read_line(&mut guess)
            .expect("failed to read line");
        if exit(&guess) {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small! "),
            Ordering::Greater => println!("{guess} is too big! "),
            Ordering::Equal => {
                println!("You win! ");
                break;
            }
        }
    }
}
fn if_else() {
    loop {
        println!("Input a number : ");

        let mut guess = String::new();
        io::stdin()
            // &mut => reference that's mutable (& = reference)
            .read_line(&mut guess)
            .expect("failed to read line");
        if exit(&guess) {
            break;
        }
        let n: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }

        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
        //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

        println!("{} -> {}", n, big_n);
    }
}

fn exit(message: &String) -> bool {
    match message.trim() {
        "e" | "exit" => return true,
        _ => return false,
    }
}

/* Shadowing is different from marking a variable as mut because we’ll get a compile-time error
* if we accidentally try to reassign to this variable without using the let keyword.
* By using let, we can perform a few transformations on a value
* but have the variable be immutable after those transformations have been completed.
*
*
*https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
* */
fn scoped_reassignment() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn testing_structs() {
    struct Hello {
        props: String,
    }
    trait World {
        fn method(self, n: usize) -> usize;
    }
    // unit struct
    struct Goodbye;
    impl World for Hello {
        fn method(self, n: usize) -> usize {
            let mut marker: usize = n;
            for i in 0..n {
                marker += 1;
                println!("{} : {} World", i, self.props);
            }

            return marker;
        }
    }
    impl World for Goodbye {
        fn method(self, n: usize) -> usize {
            println!("{} : Goodbye World", n);
            return n;
        }
    }

    let hello_world = Hello {
        props: String::from("Hello"),
    };
    let goodbye_world = Goodbye;
    // Note : Do not forget this "impl" to show we need any struct that has this trait
    fn use_method(entity: impl World) {
        entity.method(4);
    }
    use_method(hello_world);

    use_method(goodbye_world);
    // trait Limit {
    //     fn within_limit(self, n: usize) -> bool;
    // }

    // struct Unlimited;

    // impl Limit for usize {
    //     fn within_limit(self, n: usize) -> bool {
    //         n < self
    //     }
    // }

    // impl Limit for Unlimited {
    //     fn within_limit(self, _: usize) -> bool {
    //         true
    //     }
    // }
}
