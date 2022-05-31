#include <stdio.h>

int main(int argc, char **argv) {

    int i = -1;

    if (i) {
        printf("i is true\n");
    } else {
        printf("i is false\n");
    }

    switch (i) {
        case 0:
            printf("i is 0\n");
            break;
        default:
            printf("invalid value\n");
            break;
        case 1:
            printf("i is 1\n");
            break;
    }

    return 0;
}