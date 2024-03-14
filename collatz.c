#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

unsigned int collatz_length(uint64_t x) {
    unsigned int n = 0;
    while (x != 1) {
        switch (x & 1) {
        case 1:
            // x is odd.
            x = 3 * x + 1;
            break;
        case 0:
            // x is even.
            x /= 2;
            break;
        }
        n++;
    }
    return n;
}

void test_collatz_length() {
    assert(collatz_length(1) == 0);
    assert(collatz_length(3) == 7);
    // From https://en.wikipedia.org/wiki/Collatz_conjecture
    assert(collatz_length(670617279) == 986);
    assert(collatz_length(989345275647) == 1348);
}

int main(int argc, char **argv) {
    test_collatz_length();

    uint64_t start = atol(argv[1]);
    printf("%d\n", collatz_length(start));
    return 0;
}
