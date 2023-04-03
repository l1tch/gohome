# GoHome

A (sometimes) faster Sorting Algorithm inspired by Pigeonhole Sort and Countin Sort, coded in Rust.

## The Algorithm

GoHome is inspired from Pigeonhole Sort and Counting Sort; essentially, it creates an “Oarr” (Ordering Array) and cycles the dataset using the values as index for counting the element: for example, if we have the current value equal to 5, then it will access Oarr[5] and sum one; if a number is outside the size of Oarr, a new array is created and merged with Oarr.
The final array will be built using Oarr, taking the index as final value and repeating it *“Oarr[index]”* times.
