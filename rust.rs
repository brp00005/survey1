//@author Robert Coleman
// I understand academic integrity
// This JavaScript file calculates the hypercake of user input dimensions

// "-> i32" bounds the return type of the function to a 32 bit integer
fn hypercake(n: i32, k: i32) -> i32 {
	fn combination(n: i32, r: i32) -> i32{
		fn factorial(x: i32) -> i32{
            		// no need to say return in rust syntax
			if x == 0 || x== 1 {
				1			
			}
			else {
				x * factorial(x-1)
			}
		}
			// also showcasing return works
		if r >= 0 && n>=r {
			return factorial(n) / ( factorial(r) * factorial(n - r) );
		}
		else {
			return 0;
		}
	}
		
	// i and sum must be mutable since they become dynamic variables
	let mut i = 0;
	let mut sum = 0;
	while i <= k {
		sum += combination (n, i);
		i += 1;
	}
	
	return sum;
}	

fn main() {

    // Get user input
    println!("What are the number of cuts?");
    // declaring the input
    let mut cuts = String::new();
    // reading the input
    std::io::stdin().read_line(&mut cuts).expect("Failed to read line");
    // parsing input to 32 bit
    let cuts: i32 = cuts.trim().parse().expect("Invalid input");

    println!("What are the number of dimensions?");
    let mut dimensions = String::new();
    std::io::stdin().read_line(&mut dimensions).expect("Failed to read line");
    let dimensions: i32 = dimensions.trim().parse().expect("Invalid input");

	let result = hypercake(cuts, dimensions);
	println!("The result is: {}", result);
}
