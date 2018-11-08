use std::io;

fn main() {
	loop{
		println!("Input degree unit: 'C' for Celsius and 'F' for Farenheit. Press Q to quit.");
    
    	let mut unit = String::new();

		io::stdin().read_line(&mut unit)
       		.expect("Failed to read line");
    	
    	if unit.trim() == "C" || unit.trim() == "c" {   		   	
    		loop {
    			println!("Input degree in Celsius");
	
				let mut degree = String::new();

				io::stdin().read_line(&mut degree)
       				.expect("Failed to read line");
       		    
    			let degree: f32 = match degree.trim().parse() {
    				Ok(num) => num,
    				Err(_) => continue,
				};
	
				println!("The temperature in Farenheit is: {}°F",cel_to_far(degree));
				break;
			}
		} else if unit.trim() == "F" || unit.trim() == "f" {
			loop {
    			println!("Input degree in Farenheit");
	
				let mut degree = String::new();

				io::stdin().read_line(&mut degree)
       			.expect("Failed to read line");
       		    
    			let degree: f32 = match degree.trim().parse() {
    			Ok(num) => num,
    			Err(_) => continue,
			};
	
			println!("The temperature in Celcius is: {}°C",far_to_cel(degree));
			break;
			}
		} else if unit.trim() == "Q" || unit.trim() == "q" {
			std::process::exit (1);
			println!( "Quitting {}" , 1);
		}
	}
}

fn cel_to_far(deg_c: f32) -> f32 {
	let deg_f = deg_c * 1.8 + 32.0;
	return deg_f
}

fn far_to_cel(deg_f: f32) -> f32 {
	let deg_c = (deg_f - 32.0) / 1.8;
	return deg_c
}