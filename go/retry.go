package backoff

import (
	"fmt"
	"math"
	"time"
)

// ExponentialBackOff performs the function f() for retries and sleeps
// for 2^current retry * duration between each failed attempt
func ExponentialBackOff(retries int, d time.Duration, f func() error) error {
	var err error
	for i := 0; i < retries; i++ {
		if err = f(); err == nil {
			return nil
		}
		time.Sleep(time.Duration(math.Pow(2, float64(i))) * d)
	}

	if err != nil {
		return fmt.Errorf("Retry failed after %d attempts: %w", retries, err)
	}

	return nil
}
