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

	//@author: Branden Purdum
    // Execute tests from tests.rs, unit tests 
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute test");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}


//author: Branden Purdum
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn it_works() {
          // Define the sequence as a 1D array
          let sequence = [1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 1, 2, 4, 8, 15, 26, 42, 64, 93, 130, 176, 1, 2, 4, 8, 16, 31, 57, 99, 163, 256, 386, 1, 2, 4, 8, 16, 32, 63, 120, 219, 382, 638];

          let mut counter = 0; // Declare counter as mutable

          //i = k, j = n

        //goes from [2,6) (rows), and [0, 10) (columns), goes from left to right top to bottom
          for i in 2..6 {
              for j in 0..11 {
                  println!("Counter is {}", counter);
                  println!("j is: {}\n", j); 
                
                  assert_eq!(hypercake(j, i), sequence[counter]);
                  counter += 1; 
                  
              }
          }
      }
  }