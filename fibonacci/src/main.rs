use std::io;

fn main() {
    loop {
    	println!("Input Fibonacci sequence length:");
    	
    	let mut length = String::new();

		io::stdin().read_line(&mut length)
       		.expect("Failed to read line");
       	
       	let length: usize = match length.trim().parse() {
    				Ok(num) => num,
    				Err(_) => continue,
		};
					
       	let mut fibonacci = vec![1;length];
       	
       	for index in 2..length{
       		fibonacci[index] = fibonacci[index-1] + fibonacci[index-2];
       	}
       	
       	for index in &fibonacci {
       		print!("{}, ", index);
       	}
       	break
    }
}
