const readline = require('readline');

// Define a global array to serve as the cache
let cache = new Array(1000).fill(-1);

function hypercake(n, k) {
                function combination(n, r) {
                    function factorial(x) {
                        // If the result is already in the cache, return it
                        if (x >= 0 && x < 1000 && cache[x] != -1) {
                            return cache[x];
                        }

                        let result;

                        if (x == 0 || x == 1) {
                            result = 1;
                        }
                        else {
                            result = x * factorial(x - 1);
                        }

                        // Store the result in the cache
                        if (x >= 0 && x < 1000) {
                            cache[x] = result;
                        }

                        return result;
                    }

                    if (r >= 0 && n >= r) {
                        return factorial(n) / (factorial(r) * factorial(n - r));
                    }
                    else {
                        return 0;
                    }
                }

            let i = 0;
            let sum = 0;
            while (i <= k) {
                sum += combination(n, i);
                i++;
            }

    return sum;
}

// Main function
function main() {
    // Initialize the cache with -1 to indicate that values are not calculated yet
    cache.fill(-1);

    // Get user input using readline
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    rl.question("What are the number of cuts? ", (n) => {
        rl.question("What are the number of dimensions? ", (k) => {
            // Convert input to integers
            n = parseInt(n);
            k = parseInt(k);

            // Calculate and display the result
            let result = hypercake(n, k);
            console.log("The result is: " + result);

            // Close the readline interface
            rl.close();
        });
    });
}

// Call the main function
main();
