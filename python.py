def hypercake():
    def combinations(n, r):
        # Making the array structure
        cache = []

        # Memoized factorial, had to change to x instead of n
        def factorial(x):
            # Indicate that not local to scope
            nonlocal cache

            # If it exists in there, use memoization
            if len(cache) > x and cache[x] is not None:
                return cache[x]

            # If not, calculate and store
            if x == 0 or x == 1:
                result = 1
            else:
                result = x * factorial(x-1)

            # Store the result in the cache using append
            while len(cache) <= x:
                cache.append(None)
            cache[x] = result

            return result

        # Calculation for combination
        nFactorial = int(factorial(n))
        rFactorial = int(factorial(r))
        nrFactorial = int(factorial(n-r))

        return int(nFactorial / (rFactorial * nrFactorial))
        
    # Get user input, cast cuts and dimensions to avoid complaints
    cuts = int(input("Enter cut number: "))
    dimensions = int(input("Enter dimensions: "))  # Convert the input to an integer
    
    # If dimensions is 0
    if dimensions == 0:
        return 1
    
    total = 0
    
    # Iterate through nCd
    for number in range(dimensions + 1):
        total += combinations(cuts, number)

    return total

# Test the hypercake function
result = hypercake()
print(f"The result is: {result}")
