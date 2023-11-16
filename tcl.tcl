proc hypercake {} {

    #get user input
    puts -nonewline "Enter cuts: "
    flush stdout
    gets stdin cuts

    puts -nonewline "Enter dimensions: "
    flush stdout
    gets stdin dimensions

    #convert to ints 
    set cuts [expr {$cuts + 0}]
    set dimensions [expr {$dimensions + 0}]

    #counter
    set total 0
 
    #do summation
    for {set i 0} {$i <= $cuts} {incr i} {
        set total [expr {[combination $dimensions $i] + $total}]
    }

    #print result
    puts -nonewline "Result for inputs: "
    puts $total
}

#recursion 
proc factorial {n} {
    if {$n == 0 || $n == 1} {
        return 1
    } else {
        return [expr {$n * [factorial [expr $n - 1]]}]
    }
}

#n! / (r! * (n-r)!) stuff
proc combination {n r} {
    set nFactorial [factorial $n]
    set rFactorial [factorial $r]
    set nrFactorial [factorial [expr {$n - $r}]]

    return [expr {$nFactorial / ($rFactorial * $nrFactorial)}]
}

#call
hypercake
