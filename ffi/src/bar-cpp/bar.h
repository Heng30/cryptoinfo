#ifndef _BAR_H_
#define _BAR_H_
#include <iostream>

int bar_say_hello(const char *name);

class Bar {
  public:
    Bar(int base) : m_base(base) {}
    ~Bar() { std::cout << "Bar exit..." << std::endl; }
    int64_t add(int a, int b);

  private:
    int m_base = 0;
};

#endif // _BAR_H_
