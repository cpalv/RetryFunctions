# Retry functions in C, Go, and Rust

The retry functions will attempt the provided closure/function until
it succeeds or the maximum retry attempts have been exhausted.

```
retry_function(attempts, time_unit, function)
```

The retry functions will sleep for 2<sup>attempt</sup>*time unit after each
failed attempt. The final error if any is returned. 
