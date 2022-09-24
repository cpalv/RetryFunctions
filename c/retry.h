#include<unistd.h>

// time units
#define MICROSECOND 1
#define MILLISECOND  1000
#define SECOND      1000000

// errors
#define ERR_NO_ATTEMPTS -1

typedef int (*fn)(void);

// The function f should return 0 on success and 
// any non-zero for failure
int retry_fn(int attempts, unsigned int time_unit, fn f);
