// Verilator main of the Occamy simulation: the clock and reset around `testharness`, the fesvr frontend that
// loads the ELF and the boot ROM, and the memory DPI. It follows upstream's Verilator flow of snitch_cluster
// (target/common/test/tb_bin.cc and verilator_lib.cc, SHL-0.51), with three changes:
//  - the model is built with --timing (the testharness has delays: the 32.768 kHz RTC), so the main advances the
//    simulated time of the Verilator context, one picosecond per unit, 500 per half period of the 1 GHz clock;
//  - no VCD and no .rtlbinary file;
//  - the run ends with the line `Cycles: <number of rising edges of clk_i>`.
#include <svdpi.h>

#include <cassert>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <iostream>
#include <memory>

#include "Vtestharness.h"
#include "Vtestharness__Dpi.h"
#include "sim.hh"
#include "tb_lib.hh"
#include "verilated.h"

namespace sim {

// Number of half periods between HTIF checks.
const int HTIFTimeInterval = 200;
// Half period of the clock in units of the time precision (1 ps).
const uint64_t HalfPeriod = 500;

uint64_t CYCLES = 0;

void sim_thread_main(void *arg) { ((Sim *)arg)->main(); }

Sim::Sim(int argc, char **argv) : htif_t(argc, argv), ipc(argc, argv) {
    Verilated::commandArgs(argc, argv);
}

void Sim::idle() { target.switch_to(); }

/// Execute the simulation.
int Sim::run() {
    host = context_t::current();
    target.init(sim_thread_main, this);
    return htif_t::run();
}

void Sim::main() {
    auto contextp = Verilated::threadContextp();
    auto top = std::make_unique<Vtestharness>();

    bool clk_i = false;
    uint64_t half = 0;
    while (!contextp->gotFinish()) {
        clk_i = !clk_i;
        // reset for the first four cycles
        top->rst_ni = half >= 8;
        top->clk_i = clk_i;
        top->eval();
        if (clk_i) CYCLES++;
        half++;
        contextp->timeInc(HalfPeriod);
        // Switch to the HTIF interface in regular intervals.
        if (half % HTIFTimeInterval == 0) {
            host->switch_to();
        }
    }
    top->final();
}

}  // namespace sim

// DPI calls.
void tb_memory_read(long long addr, int len, const svOpenArrayHandle data) {
    void *data_ptr = svGetArrayPtr(data);
    assert(data_ptr);
    sim::MEM.read(addr, len, (uint8_t *)data_ptr);
}

void tb_memory_write(long long addr, int len, const svOpenArrayHandle data,
                     const svOpenArrayHandle strb) {
    const void *data_ptr = svGetArrayPtr(data);
    const void *strb_ptr = svGetArrayPtr(strb);
    assert(data_ptr);
    assert(strb_ptr);
    sim::MEM.write(addr, len, (const uint8_t *)data_ptr,
                   (const uint8_t *)strb_ptr);
}

int main(int argc, char **argv) {
    auto sim = std::make_unique<sim::Sim>(argc, argv);
    int code = sim->run();
    std::cout << "Cycles: " << sim::CYCLES << std::endl;
    return code;
}
