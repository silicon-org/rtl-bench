# Constraints

A clock period comes from the first of these that exists, and its `source` field says which:

1. An SDC shipped by upstream for the variant.
2. A frequency published for the design (paper, data sheet, tape-out report), cited with page.
3. A scaling rule stated in this file, applied to 1 or 2 of another technology.

I/O delays are a fraction of the period of the launching or capturing clock (`input_delay_fraction`, `output_delay_fraction`); the fraction is stated per variant.

The SDC is generated from `design.toml` by `rtl-bench sdc`; upstream SDC files are kept unmodified next to it.
