
module DifftestTriggerCSRState(
  input         clock,
  input         enable,
  input  [63:0] io_tselect,
  input  [63:0] io_tdata1,
  input  [63:0] io_tinfo,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_TriggerCSRState (
  input   longint io_tselect,
  input   longint io_tdata1,
  input   longint io_tinfo,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_TriggerCSRState (io_tselect, io_tdata1, io_tinfo, io_coreid);
  end
`endif
`endif
endmodule
