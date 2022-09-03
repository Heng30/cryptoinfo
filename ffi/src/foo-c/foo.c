#include <stdio.h>

#include "foo.h"

int foo_say_hello(const char *name) {
    printf("Hi %s, I'm foo.\n", name);
    return 0;
}
