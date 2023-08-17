# Purpose
benchmark multiple implementations of knapsack problem algorithm against each other.


# Results
The code drops out the slowest algorithm on each test iteration. The amount of data provided goes up with each test iteration.

Notice that the algorithms return the same answer for all sizes.

```
*** Parameters ***
# items:        10
Total value:    843
Total weight:   534
Allowed weight: 267

---------- exhaustive: 10
Elapsed: 264.53µs
Value: 673 Weight: 265

---------- branch: 10
Elapsed: 45.995µs
Value: 673 Weight: 265

---------- rods: 10
Elapsed: 40.473µs
Value: 673 Weight: 265

---------- dynamic: 10
Elapsed: 175.996µs
Value: 673 Weight: 265

*** Parameters ***
# items:        28
Total value:    2612
Total weight:   1907
Allowed weight: 953

---------- branch: 28
Elapsed: 339.432743ms
Value: 2082 Weight: 947

---------- rods: 28
Elapsed: 37.777066ms
Value: 2082 Weight: 947

---------- dynamic: 28
Elapsed: 2.010061ms
Value: 2082 Weight: 947

*** Parameters ***
# items:        39
Total value:    2947
Total weight:   2823
Allowed weight: 1411

---------- rods: 39
Elapsed: 6.060983193s
Value: 2333 Weight: 1401

---------- dynamic: 39
Elapsed: 3.627458ms
Value: 2333 Weight: 1401

*** Parameters ***
# items:        200
Total value:    15874
Total weight:   13473
Allowed weight: 6736

---------- dynamic: 200
Elapsed: 94.952155ms
Value: 13073 Weight: 6735
```
