{
  description = "Open RTL benchmark designs";

  # nixpkgs and flake-utils are pinned by flake.lock.
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # slang v12.1 (MIT). nixpkgs carries slang 12.0 or older, and slang 7.0, which cannot elaborate
        # generate loops that count down to 0 from a parameter expression (wally cacheLRU.sv).
        slang = pkgs.sv-lang.overrideAttrs (_: {
          version = "12.1";
          src = pkgs.fetchFromGitHub {
            owner = "MikePopoloski";
            repo = "slang";
            rev = "42710c788ce76cd08ce2080773e0b8286c428a05";
            hash = "sha256-8WffLY3IhsCkiYUMfe2aZy1zzOqEBHX4MTMai2dZFI0=";
          };
        });
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
        # The nixpkgs newlib is built for the medium-low code model, which cannot address the
        # 0x80000000 DRAM of blackparrot: build it with -mcmodel=medany. The crtbegin.o and crtend.o of
        # the nixpkgs gcc have the same problem and only run C++ and TM constructors, so the
        # programs link empty ones.
        newlibMedany = pkgs.pkgsCross.riscv64-embedded.newlib.overrideAttrs (o: {
          preConfigure = o.preConfigure + ''
            export CFLAGS_FOR_TARGET="-g -O2 -mcmodel=medany"
          '';
        });
        emptyCrt = pkgs.runCommand "empty-crt" { } ''
          mkdir $out
          : > empty.s
          ${rv.binutils}/bin/riscv64-none-elf-as -march=rv64gc -mabi=lp64d empty.s -o $out/crtbegin.o
          cp $out/crtbegin.o $out/crtend.o
        '';
        # The host side of Vortex's rtlsim: Berkeley SoftFloat 3 with the options of
        # third_party/Makefile, and Ramulator 2 with the GDDR6 model that Vortex adds (its
        # third_party/Makefile). The commits are those of the Vortex submodules; the three
        # libraries that Ramulator fetches at configure time are pinned to the tags it names.
        vxSoftfloat = pkgs.stdenv.mkDerivation {
          pname = "berkeley-softfloat-3-vortex";
          version = "b51ef8f";
          src = pkgs.fetchFromGitHub {
            owner = "ucb-bar"; repo = "berkeley-softfloat-3";
            rev = "b51ef8f3201669b2288104c28546fc72532a1ea4";
            hash = "sha256-RSTTkJyLhpZO8N/cFar4oxkbt8m4FTAGnsPSDBvvGXQ=";
          };
          buildPhase = ''
            cd build/Linux-x86_64-GCC
            make SPECIALIZE_TYPE=RISCV SOFTFLOAT_OPTS="-fPIC -DSOFTFLOAT_ROUND_ODD -DINLINE_LEVEL=5 -DSOFTFLOAT_FAST_DIV32TO16 -DSOFTFLOAT_FAST_DIV64TO32 -DTHREAD_LOCAL=_Thread_local"
          '';
          installPhase = ''
            mkdir -p $out/lib $out/include
            cp softfloat.a $out/lib/
            cp -r ../../source/include/* $out/include/
            # the sources include ../RISCV/specialize.h relative to the include directory
            cp -r ../../source/RISCV $out/RISCV
          '';
        };
        vxRamulator = pkgs.stdenv.mkDerivation {
          pname = "ramulator2-vortex";
          version = "e62c84a";
          src = pkgs.fetchFromGitHub {
            owner = "CMU-SAFARI"; repo = "ramulator2";
            rev = "e62c84a6f0e06566ba6e182d308434b4532068a5";
            hash = "sha256-CLbPVvz6eMhmVsL8FV9Vc6ySm+/rSVvMN6SR6AMwoKA=";
          };
          nativeBuildInputs = [ pkgs.cmake ];
          postPatch = ''
            mkdir -p ext
            cp -r ${pkgs.fetchFromGitHub { owner = "jbeder"; repo = "yaml-cpp"; rev = "0579ae3d976091d7d664aa9d2527e0d0cff25763"; hash = "sha256-2tFWccifn0c2lU/U1WNg2FHrBohjx8CXMllPJCevaNk="; }} ext/yaml-cpp
            cp -r ${pkgs.fetchFromGitHub { owner = "gabime"; repo = "spdlog"; rev = "ad0e89cbfb4d0c1ce4d097e134eb7be67baebb36"; hash = "sha256-kA2MAb4/EygjwiLEjF9EA7k8Tk//nwcKB1+HlzELakQ="; }} ext/spdlog
            cp -r ${pkgs.fetchFromGitHub { owner = "p-ranav"; repo = "argparse"; rev = "997da9255618311d1fcb0135ce86022729d1f1cb"; hash = "sha256-vbf4kePi5gfg9ub4aP1cCK1jtiA65bUS9+5Ghgvxt/E="; }} ext/argparse
            chmod -R u+w ext
            # yaml-cpp 0.7.0 misses this include, which newer compilers need
            sed -i '1i #include <cstdint>' ext/yaml-cpp/src/emitterutils.cpp
          '';
          cmakeFlags = [ "-DFETCHCONTENT_FULLY_DISCONNECTED=ON" "-DCMAKE_POLICY_VERSION_MINIMUM=3.5" ];
          postBuild = ''
            g++ -std=c++20 -O3 -DNDEBUG -fPIC -isystem ../src -isystem ../ext/spdlog/include -isystem ../ext/yaml-cpp/include \
              -c ../src/dram/impl/GDDR6.cpp -o gddr6.o
            g++ -shared gddr6.o -L.. -lramulator -o ../libramulator-vortex.so
          '';
          installPhase = ''
            mkdir -p $out/lib $out/src $out/spdlog $out/yaml-cpp
            cp ../libramulator.so ../libramulator-vortex.so $out/lib/
            cp -r ../src/* $out/src/
            cp -r ../ext/spdlog/include $out/spdlog/include
            cp -r ../ext/yaml-cpp/include $out/yaml-cpp/include
          '';
        };
        rv32 = pkgs.pkgsCross.riscv32-embedded.buildPackages;
        # Vortex's makefiles call the 32-bit tools riscv32-unknown-elf-*.
        riscv32Aliases = pkgs.runCommand "riscv32-unknown-elf-aliases" { } ''
          mkdir -p $out/bin
          ln -s ${rv32.gcc}/bin/riscv32-none-elf-gcc $out/bin/riscv32-unknown-elf-gcc
          for tool in as ld nm objcopy objdump readelf; do
            ln -s ${rv32.binutils}/bin/riscv32-none-elf-$tool $out/bin/riscv32-unknown-elf-$tool
          done
        '';
        # OpenTitan builds its software with Bazel, whose hermetic toolchains are dynamically linked
        # programs and whose actions use /bin/bash and srec_cat: a FHS environment for them.
        otFhs = pkgs.buildFHSEnv {
          name = "otfhs";
          targetPkgs = pk: with pk; [
            bash coreutils gnugrep gnused gawk findutils diffutils gnutar gzip xz bzip2 unzip zip git
            curl wget python3 gcc glibc zlib openssl libxcrypt-legacy ncurses libelf which file patch
            perl rsync openjdk21_headless srecord
          ];
          runScript = "bash";
        };
        # The wally testbench runs bin/extractFunctionRadix.sh, whose shebang is /bin/bash;
        # NixOS has no /bin/bash, so run it with bash from the shell.
        wallyShims = pkgs.writeShellScriptBin "extractFunctionRadix.sh" ''
          exec ${pkgs.bash}/bin/bash "$WALLY/bin/extractFunctionRadix.sh" "$@"
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          # libelf, which the OpenTitan simulation links
          buildInputs = [ pkgs.elfutils ];
          packages = with pkgs; [
            verilator
            slang
            cargo
            rustc
            clippy
            rustfmt
            git
            gnumake
            python3
            uv
            gettext
            rv.gcc
            rv.binutils
            riscvAliases
            rv32.gcc
            rv32.binutils
            riscv32Aliases
            otFhs
            # Chipyard's generation flow (JDK 17: the Scala 2.12 of sbt cannot read newer class files)
            jdk17
            dtc
            perl
            gawk
            which
            wallyShims
          ];
          # uv runs upstream scripts (elf2hex) with the Python above, never downloads one.
          UV_PYTHON_DOWNLOADS = "never";
          RISCV_NEWLIB_MEDANY = "${newlibMedany}/riscv64-none-elf/lib";
          RISCV_EMPTY_CRT = "${emptyCrt}";
          VX_SOFTFLOAT = "${vxSoftfloat}";
          VX_RAMULATOR = "${vxRamulator}";
          # libfesvr and libriscv (Spike) for the Chipyard simulator
          CY_SPIKE = "${pkgs.spike}";
        };
      });
}
