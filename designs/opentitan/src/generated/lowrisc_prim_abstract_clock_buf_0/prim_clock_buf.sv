// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// This file is auto-generated.
// Used parser: Fallback (regex)


// This is to prevent AscentLint warnings in the generated
// abstract prim wrapper. These warnings occur due to the .*
// use. TODO: we may want to move these inline waivers
// into a separate, generated waiver file for consistency.
//ri lint_check_off OUTPUT_NOT_DRIVEN INPUT_NOT_READ HIER_BRANCH_NOT_READ
module prim_clock_buf

#(

  // Turning off these verilator lints because keeping these parameters makes it consistent with
  // the IP in hw/ip/prim_xilinx/rtl/ .
  /* verilator lint_off UNUSED */
  parameter bit NoFpgaBuf = 1'b0, // serves no function in generic
  parameter bit RegionSel = 1'b0  // serves no function in generic
  /* verilator lint_on UNUSED */

) (
  input clk_i,
  output logic clk_o
);

  if (1) begin : gen_generic
    prim_generic_clock_buf #(
      .NoFpgaBuf(NoFpgaBuf),
      .RegionSel(RegionSel)
    ) u_impl_generic (
      .*
    );

  end

endmodule
//ri lint_check_on OUTPUT_NOT_DRIVEN INPUT_NOT_READ HIER_BRANCH_NOT_READ
