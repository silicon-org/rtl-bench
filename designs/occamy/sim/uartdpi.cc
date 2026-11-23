// The UART DPI of Occamy (target/sim/test/uartdpi/uartdpi.c, upstream's file, compiled unchanged) with C
// linkage. Verilator compiles every source file as C++, and the DPI functions of the model are C functions;
// upstream's header has no `extern "C"`.
extern "C" {
#include "uartdpi.c"
}
