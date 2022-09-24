#include<stdlib.h>
#include<stdio.h>
#include<unistd.h>
#include<errno.h>

#include "retry.h"

int retry_fn(int attempts, unsigned int time_unit, fn f) {
	int urc, rc;
	urc = rc = 0;
	unsigned int power = 1;

	if (attempts < 1) 
		return ERR_NO_ATTEMPTS;

	for(int i = 0; i < attempts; i++) {
		//printf("attempt: %d\n", i);
		rc = f();
		if (!rc)
			return rc;

		// snooze for 2^i * time_unit
		urc = usleep(power * time_unit);
		if (urc == -1) {
			urc = errno;
			return urc;
		}

		//printf("2^%d = %d\n", i, power);
		power = power << 1;
	}
	return rc;
}
