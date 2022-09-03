#include "bar.h"

int bar_say_hello(const char* name) {
    std::cout << "Hi " << std::string(name) << ", I'm bar." << std::endl;
    return 0;
}
