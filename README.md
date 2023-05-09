# Retry functions in C, Go, and Rust

The retry functions will attempt the provided closure/function until
it succeeds or the maximum retry attempts have been exhausted.

```
retry_function(attempts, time_unit, function)
```

The retry functions will sleep for 2<sup>attempt</sup>*time unit after each
failed attempt. The final error if any is returned. 

## An argument against the Rust compiler's amazing error messages

It's been a minute since the last commit to this repo, but because
the Rust compiler's error messages is a common selling point.  I thought
I'd take the time to say that they suck.  In specfic scenerios, like the one
I encountered when writing the Rust version of the retry function.  The Rust
compiler's error messages were entirely unhelpful, and I spent an unknown
quantity of my life listening to the compiler who was sending me off 
on the wildest of goose chases.

Granted, this exercise was my first real attempt at not writing a "Hello world"
application in Rust; and unfortunately, since I did not commit every 
thought/recommendation from the compiler to fix the problems encountered.
There isn't really a good way to point to the specific examples.

Working from the volatile memory in my smooth human brain and all caveats considered.
The compiler kept suggesting to add different traits(? I don't remember if they
were traits, but it definately kept asking me to add things) to the generic parameters.

So the secret to the Rust compiler's error messages?  Throw stuff at the wall
and see what sticks.

It wasn't until I threw the compiler out the window and really thought
about the problem; that I managed to write something that resembled a solution.  

So it turns out, sometimes you have still have to understand the problem to come
up with a reasonable resolution.


## Latency Table

Approximations for min-maxing usage of retry functions

| Time Range | Example operations                                       |
|------------|----------------------------------------------------------|
| <1ns       | Accessing CPU registers                                  |
| 1-10ns     | L1/L2 cache access / Branch mispredict                   |
| 10-100ns   | L3 cache access                                          |
| 100-1000ns | System call                                              |
| 1-10us     | Process context switch                                   |
| 10-100us   | Process an HTTP request / SSD sequential read            |
| 100-1000us | SSD write / Inter-zone network round trip                |
| 1-10ms     | Intra-zone network / HDD seek                            |
| 10-100ms   | Network round trip US East to West / 1GB read from RAM   |
| 100-1000ms | Slow hash functions / TLS handshake / 1GB read from SSD  |
| >1s        | 1GB network transfer within same region                  |


### Citation for table

“Latency Numbers Programmer Should Know: Crash Course System Design #1.” YouTube, 4 Oct. 2022, www.youtube.com/watch?v=FqR5vESuKe0. 
