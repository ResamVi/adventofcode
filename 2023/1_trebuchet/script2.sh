#!/bin/bash

sum=0
while read line; do
    # We need avoid things like "oneighthree", "twone" not 
    # getting substituted to "183", "21"
    translated=$(echo $line | sed \
        -e "s/one/o1e/g" \
        -e "s/two/t2o/g" \
        -e "s/three/t3e/g" \
        -e "s/four/f4r/g" \
        -e "s/five/f5e/g" \
        -e "s/six/s6x/g" \
        -e "s/seven/s7n/g" \
        -e "s/eight/e8t/g" \
        -e "s/nine/n9e/g")

    # Use Perl-style regex to find numbers (grep). 
    # Join the grep result to a single line (xargs).
    # Remove the whitespace between the joined result (tr).
    var=$(echo $translated | grep -Po \\d | xargs | tr -d '[:blank:]')

    # Get first and last digit found.
    number=${var:0:1}${var: -1}
    (( sum += number ))
done

echo Result: ${sum}
