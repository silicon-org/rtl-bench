
module DifftestArchEvent(
  input         clock,
  input         enable,
  input         io_valid,
  input  [31:0] io_interrupt,
  input  [31:0] io_exception,
  input  [63:0] io_exceptionPC,
  input  [31:0] io_exceptionInst,
  input         io_hasNMI,
  input         io_virtualInterruptIsHvictlInject,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_ArchEvent (
  input       int io_interrupt,
  input       int io_exception,
  input   longint io_exceptionPC,
  input       int io_exceptionInst,
  input       bit io_hasNMI,
  input       bit io_virtualInterruptIsHvictlInject,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_ArchEvent (io_interrupt, io_exception, io_exceptionPC, io_exceptionInst, io_hasNMI, io_virtualInterruptIsHvictlInject, io_coreid);
  end
`endif
`endif
endmodule
