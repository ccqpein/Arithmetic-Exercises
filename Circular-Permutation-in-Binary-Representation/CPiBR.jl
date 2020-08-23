circularPermutation(n, start) = map(x -> start ⊻ x ⊻ (x >> 1) ,0:((1<<n) - 1) )
