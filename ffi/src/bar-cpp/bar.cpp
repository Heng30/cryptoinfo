#include "bar.h"

int bar_say_hello(const char *name) {
    std::cout << "Hi " << std::string(name) << ", I'm bar." << std::endl;
    return 0;
}

int64_t Bar::add(int a, int b) { return m_base + a + b; }
