
use std::io;
use std::i32;
use std::ops::Add;

struct Number {
    chunks: Vec<i32>
}

impl Number {

    fn new(x: i32) -> Number {
        Number{chunks: vec![x]}
    }

    fn format(n: Number) -> String {
        let chunks: Vec<_> = n.chunks.iter().map(|n| n.to_string()).collect();
        chunks.join("")
    }

    fn equate_sizes(&mut self, &mut other: Number) {
        let delta = (self.chunks.len() - other.chunks.len()) as i32;
        if delta < 0 {
            for i in 0..-delta {
                self.chunks.push(0);
            }
        } else {
            for i in 0..delta {
                other.chunks.push(0);
            }
        }
    }
}



impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let pairs = self.chunks.iter().zip(other.chunks.iter());
        Number::equate_sizes(&mut self, other);
        let mut result = vec![];
        let mut overflow = false;
        for (x1, x2) in pairs {
            let mut sum = (x1 + x2) as u32;
            sum = if overflow {
                sum + 1
            } else {
                sum
            };
            overflow = if sum > i32::MAX as u32 { true } else { false };
            sum = if overflow {
                sum - (i32::MAX as u32)
            } else {
                sum
            };
            result.push(sum as i32);
        }
        Number{chunks: result}
    }
}

// TODO how number is split in chunks?

// fn fibonacci_fast(n: i32) -> Number {
//     if n < 2 {
//         Number::new(n)
//     } else {
//         fib(n)
//     }
// }

// fn fib(n: i32) -> Number {
//     // TODO
//     let mut result: Vec<Number> = Vec::new();
//     result.push(Number::new(0));
//     result.push(Number::new(1));
//     for i in 2 .. (n as usize) + 1 {
//         result.push(result[i-1] + result[i-2]);
//     }
//     result[n as usize]
// }

fn test_sum() -> Number{
    let num1 = Number{
        chunks: vec![0, 1, 2]
    };
    let num2 = Number {
        chunks: vec![10]
    };
    num1 + num2
}

fn main() {
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input);
    // let n: i32 = input.trim().parse().unwrap();
    // print!("{}\n", Number::format(fibonacci_fast(n)))
    let x = test_sum();
    print!("{}\n", Number::format(x));

}