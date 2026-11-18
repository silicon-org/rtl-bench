
module DifftestFpCSRState(
  input         clock,
  input         enable,
  input  [63:0] io_fcsr,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_FpCSRState (
  input   longint io_fcsr,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_FpCSRState (io_fcsr, io_coreid);
  end
`endif
`endif
endmodule
