#!/bin/bash

sum=0
while read line; do
    # Use Perl-style regex to find numbers (grep). 
    # Join the grep result to a single line (xargs).
    # Remove the whitespace between the joined result (tr).
    var=$(echo $line | grep -Po \\d | xargs | tr -d '[:blank:]')
    
    # Get first and last digit found.
    number=${var:0:1}${var: -1}
    (( sum += number ))

    echo $number
done

echo Result: ${sum}
