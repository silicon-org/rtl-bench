{
  description = "Open RTL benchmark designs";

  # nixpkgs and flake-utils are pinned by flake.lock.
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rv = pkgs.pkgsCross.riscv64-embedded.buildPackages;
        # Upstream scripts and makefiles call the tools riscv64-unknown-elf-*; the
        # bare-metal toolchain from nixpkgs names them riscv64-none-elf-*.
        riscvAliases = pkgs.runCommand "riscv64-unknown-elf-aliases" { } ''
          mkdir -p $out/bin
          ln -s ${rv.gcc}/bin/riscv64-none-elf-gcc $out/bin/riscv64-unknown-elf-gcc
          for tool in as ld nm objcopy objdump readelf; do
            ln -s ${rv.binutils}/bin/riscv64-none-elf-$tool $out/bin/riscv64-unknown-elf-$tool
          done
        '';
        # The wally testbench runs bin/extractFunctionRadix.sh, whose shebang is /bin/bash;
        # NixOS has no /bin/bash, so run it with bash from the shell.
        wallyShims = pkgs.writeShellScriptBin "extractFunctionRadix.sh" ''
          exec ${pkgs.bash}/bin/bash "$WALLY/bin/extractFunctionRadix.sh" "$@"
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            verilator
            sv-lang
            cargo
            rustc
            clippy
            rustfmt
            git
            gnumake
            python3
            uv
            rv.gcc
            rv.binutils
            riscvAliases
            wallyShims
          ];
          # uv runs upstream scripts (elf2hex) with the Python above, never downloads one.
          UV_PYTHON_DOWNLOADS = "never";
        };
      });
}
