cmake_minimum_required(VERSION 3.6)

add_executable(vm
  main.cpp
  cpu.cpp
  disasm.cpp
  io.cpp
  dump.cpp
  int_handler.cpp
  vm.cpp
)
