# Constraints

A clock period is given for an open PDK only, and only when a source publishes it for that PDK: `period_ps` is keyed by one of `asap7`, `sky130`, `gf180mcu`, `ihp-sg13g2`, `freepdk45`, and the `source` of the clock says where the period comes from:

1. An SDC shipped by upstream for the variant, for that PDK.
2. A frequency published for the design on that PDK (paper, data sheet, tape-out report), cited with page.

Without such a source there is no period, and `rtl-bench sdc` fails with a message that names the clock. There is no scaling between technologies.

A frequency that is published for any other process, a commercial one included, is not a technology key. It goes in a `[[variant.clock.reference]]` record of the clock (`frequency_mhz`, `process` as the source writes it, `source`), where the process is named and nothing else uses it.

I/O delays are a fraction of the period of the launching or capturing clock (`input_delay_fraction`, `output_delay_fraction`); the fraction is stated per variant and has a `source`.

The SDC is generated from `design.toml` by `rtl-bench sdc`; upstream SDC files are kept unmodified next to it.
