proc hypercake {} {

    #get user input for both things, using puts and gets 
    
    #print out with no return 
    puts -nonewline "Enter cuts: "

    #read in
    gets stdin cuts

    #prompt again
    puts -nonewline "Enter dimensions: "
    gets 

}

#brief: recursive factorial function
proc factorial{n}{
    #if 0 or 1, base case
    if{$n == 0 || $n == 1}{
        return 1
    }
    else{
        return factorial(n-1)
    }

}

#brief: calculate combinations using factorial, no memoization necessary
proc combination{n r}{
    nFactorial = factorial(n)
    rFactorial = factorial(r)
    nrFactorial = factorial(n-r)

    return nFactorial / (rFactorial * nrFactorial)

}