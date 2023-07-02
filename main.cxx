#include <stdio.h>
#include <iostream>
#include "vectors.h"

#define PROJECT_NAME "raytrace"

int main(int argc, char** argv) {
    if (argc != 1) {
        printf("%s takes no arguments.\n", argv[0]);
        return 1;
    }
    printf("This is project %s.\n", PROJECT_NAME);

    using Vector3D = Vector<double, 3>;

    Vector3D v({1., 2., 3.});
    std::cout << v.to_string() << "\n";

    return 0;
}
