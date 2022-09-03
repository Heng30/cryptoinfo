# 设置cmake最低版本
cmake_minimum_required(VERSION 3.10 FATAL_ERROR)

# 设置项目信息
project(bar-cpp LANGUAGES CXX VERSION 1.0.0)

# 设置C++的版本
set(CMAKE_CXX_STANDARD 17)

# 编译参数
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O2")

# 添加动态库
add_library(bar SHARED ${CMAKE_SOURCE_DIR}/bar.cpp)

# 添加头文件搜索目录
target_include_directories(bar PRIVATE ${CMAKE_SOURCE_DIR})

