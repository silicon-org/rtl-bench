
module DifftestNonRegInterruptPendingEvent(
  input         clock,
  input         enable,
  input         io_valid,
  input         io_platformIRPMeip,
  input         io_platformIRPMtip,
  input         io_platformIRPMsip,
  input         io_platformIRPSeip,
  input         io_platformIRPStip,
  input         io_platformIRPVseip,
  input         io_platformIRPVstip,
  input         io_localCounterOverflowInterruptReq,
  input  [ 7:0] io_coreid
);
`ifndef SYNTHESIS
`ifdef DIFFTEST

import "DPI-C" function void v_difftest_NonRegInterruptPendingEvent (
  input       bit io_platformIRPMeip,
  input       bit io_platformIRPMtip,
  input       bit io_platformIRPMsip,
  input       bit io_platformIRPSeip,
  input       bit io_platformIRPStip,
  input       bit io_platformIRPVseip,
  input       bit io_platformIRPVstip,
  input       bit io_localCounterOverflowInterruptReq,
  input      byte io_coreid
);


  always @(posedge clock) begin
    if (enable)
      v_difftest_NonRegInterruptPendingEvent (io_platformIRPMeip, io_platformIRPMtip, io_platformIRPMsip, io_platformIRPSeip, io_platformIRPStip, io_platformIRPVseip, io_platformIRPVstip, io_localCounterOverflowInterruptReq, io_coreid);
  end
`endif
`endif
endmodule
