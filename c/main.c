#include<stdlib.h>
#include<stdio.h>
#include<unistd.h>
#include<errno.h>
#include<time.h>

#include "retry.h"

int wild_func() {
	int something_else_went_wrong = 3;
	return something_else_went_wrong;
}

int successful_func(){
	return 0;
}

int main(void) {

	int val = 0;
	unsigned int time_unit = SECOND;

	int some_func() {
		srandom((unsigned int)time(NULL));
		sleep(2);
		val = rand();
		printf("current val: %d\n", val);
		return (val % 2);
	}

	int another_func() {
		int something_went_wrong = -1;
		return something_went_wrong;
	}

	printf("compiles?\n");
	
	if (!retry_fn(3, SECOND, some_func))
		printf("success!\n");
	else
		printf("failure..\n");

	printf("val: %d\n", val);

	if (!retry_fn(3, SECOND, another_func))
		printf("success!\n");
	else
		printf("failure..\n");

	if (!retry_fn(3, SECOND, wild_func))
		printf("success!\n");
	else
		printf("failure..\n");

	if (!retry_fn(3, SECOND, successful_func))
		printf("success!\n");
	else
		printf("failure..\n");


}
