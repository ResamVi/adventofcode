#!/bin/bash

translate() {
    result=$1

    # We cannot substitute everything at once.
    # E.g. "eightwothree" has multiple solutions depending on what is substituted first.
    # So use a sliding window.
    for (( i=0; i<${#1}; i++ )); do
        # Check three digit numbers.
        three=${1:$i:3}
        case $three in
            "one") 
                result=$(echo $result | sed "s/one/1/")
                ;;
            "two") 
                result=$(echo $result | sed "s/two/2/")
                ;;
            "six") 
                result=$(echo $result | sed "s/six/6/")
                ;;
        esac

        # Check four digit numbers.
        four=${1:$i:4}
        case $four in
            "four") 
                result=$(echo $result | sed "s/four/4/")
                ;;
            "five") 
                result=$(echo $result | sed "s/five/5/")
                ;;
            "nine") 
                result=$(echo $result | sed "s/nine/9/")
                ;;
        esac

        # Check five digit numbers.
        five=${1:$i:5}
        case $five in
            "three") 
                result=$(echo $result | sed "s/three/3/")
                ;;
            "seven") 
                result=$(echo $result | sed "s/seven/7/")
                ;;
            "eight") 
                result=$(echo $result | sed "s/eight/8/")
                ;;
        esac
    done

    echo $result
}


sum=0
while read line; do
    translated=$(translate $line)

    # Use Perl-style regex to find numbers (grep). 
    # Join the grep result to a single line (xargs).
    # Remove the whitespace between the joined result (tr).
    var=$(echo $translated | grep -Po \\d | xargs | tr -d '[:blank:]')

    # Get first and last digit found.
    number=${var:0:1}${var: -1}
    (( sum += number ))

    echo $number
done

echo Result: ${sum}
