
module DifftestCMOInvalEvent(
  input         clock,
  input         enable,
  input         io_valid,
  input  [63:0] io_addr,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_CMOInvalEvent (
  input   longint io_addr,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_CMOInvalEvent (io_addr, io_coreid);
  end
`endif
`endif
endmodule
