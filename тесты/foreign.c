#include <stdio.h>

void print_message(void) {
    printf("    This message originates from C\n");
    fflush(stdout);
}

void print_floats(float x, float y, float z) {
    printf("    Floats: %f, %f, %f\n", x, y, z);
    fflush(stdout);
}

int get_integer(int a, int b, int c) {
    return a*a + b*b + c*c;
}

float get_float(float a) {
    return 34.0 + 70.0*a;
}

float perform_calculation(float a, int b, float c, float d, int e, int f) {
    int ints = b*e*f;
    float floats = a+c+d;

    return (float)ints / floats;
}
