
use std::env;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    println!("args are: {:?}", args);
    if args.len() != 2
    {
        usage();
        return;
    }

    let f = args[1].parse::<f32>();
    if f.is_err()
    {
        println!("Invalide argument {}", args[1]);
        usage();
        return;
    }

    let square = f.unwrap();
    println!("Calculating the square root of {}", square);
    println!("Square Root of {} is {}", square, square_root(square));
}

fn usage()
{
    println!("Usage");
    println!("-----");
    println!("rust_first #");
    println!("outputs the square root of #");
}

fn square_root(square: f32) -> f32 {
    let mut temp: f32 = square;
    let mut root: f32;
    let mut count:i32 = 0;
    loop
    {
        count += 1;
        root = 0.5 * (temp + (square / temp));
        if (root - temp).abs() < 0.001
        {
            break;
        }

        temp = root;
    }
    println!("square root took {} iterations", count);
    root
}


