#making the array structure
cache = []

def combinations(n,r):
    nFactorial = int(factorial(n)); 
        #memoized factorial
        def factorial(n):
            # if it exists in there
            if len(cache) > n and cache[n] is not None:
                return cache[n]
            
            # if not, calculate and store 
            if n == 0 or n == 1:
                result = 1
            else:
                result = n * factorial(n-1)
            
            # Store the result in the cache using append
            while len(cache) <= n:
                cache.append(None)
            cache[n] = result
            
            return result

# Test the factorial function
result = factorial(5)
print(f"The factorial of 5 is: {result}")
