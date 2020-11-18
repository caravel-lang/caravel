#include <stdio.h>
#include <stdbool.h>

void debug_int(int out) {
    printf("%d\n", out);
}

void debug_float(double out) {
    printf("%f\n", out);
}

void debug_bool(bool out) {
    printf("%s\n", out ? "true" : "false");
}

void debug_char(char out) {
    printf("%c\n", out);
}