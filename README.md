# Retry functions in C, Go, and Rust

The retry functions will attempt the provided closure/function until
it succeeds or the maximum retry attempts have been exhausted.

```
retry_function(attempts, time_unit, function)
```

The retry functions will sleep for 2<sup>attempt</sup>*time unit after each
failed attempt. The final error if any is returned. 

## Latency Table

Approximations for min-maxing usage of retry functions.  For example, if
a network connection is dropped.  You a could use ms as the time unit
to retry the connection if the remote host is intra-zone.

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
