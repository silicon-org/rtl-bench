
module DifftestAtomicEvent(
  input         clock,
  input         enable,
  input         io_valid,
  input  [63:0] io_addr,
  input  [63:0] io_data,
  input  [ 7:0] io_mask,
  input  [ 7:0] io_fuop,
  input  [63:0] io_out,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_AtomicEvent (
  input   longint io_addr,
  input   longint io_data,
  input      byte io_mask,
  input      byte io_fuop,
  input   longint io_out,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_AtomicEvent (io_addr, io_data, io_mask, io_fuop, io_out, io_coreid);
  end
`endif
`endif
endmodule
