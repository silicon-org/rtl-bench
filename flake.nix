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
        # Verilator 4.228 (LGPL-3.0-only OR Artistic-2.0), the version that the NaxRiscv testbench pins
        # (Makefile: VERILATOR_VERSION_NAX); its main.cpp uses the Verilator 4 model layout (top->NaxRiscv).
        verilator4 = pkgs.verilator.overrideAttrs (_: {
          version = "4.228";
          src = pkgs.fetchFromGitHub {
            owner = "verilator";
            repo = "verilator";
            rev = "v4.228";
            hash = "sha256-ToYad8cvBF3Mio5fuT4Ce4zXbWxFxd6smqB1TxvlHao=";
          };
          patches = [ ];
          postPatch = "";
          doCheck = false;
        });
        # The Spike of the NaxRiscv testbench (it links libriscv, libfesvr, libdisasm, libsoftfloat, libfdt and the
        # spike object): the SpinalHDL fork, the commit of ext/riscv-isa-sim of NaxRiscv, built as ci/install-libsdl-elfio-spikespinalhdl.sh
        # does. The whole source and build tree is the output: the testbench includes its headers.
        naxSpike = pkgs.stdenv.mkDerivation {
          pname = "riscv-isa-sim-spinalhdl";
          version = "5b37ab6";
          src = pkgs.fetchFromGitHub {
            owner = "SpinalHDL"; repo = "riscv-isa-sim";
            rev = "5b37ab699f6176ea7f2d2b3187f75625eca7e511";
            hash = "sha256-30ufkwqmZwlO2Wij77ZPDrK1VBp3UraiGQqtKPX/mss=";
          };
          nativeBuildInputs = [ pkgs.dtc ];
          dontConfigure = true;
          dontFixup = true;
          buildPhase = ''
            mkdir build && cd build
            ../configure --prefix=$out --without-boost --without-boost-asio --without-boost-regex
            make -j$NIX_BUILD_CORES
            g++ --shared -L. -Wl,--export-dynamic -o package.so spike.o libspike_main.a libriscv.a \
              libdisasm.a libsoftfloat.a libfesvr.a libfdt.a -lpthread -ldl
            cd ..
          '';
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
          '';
        };
        # ELFIO at the commit that the NaxRiscv Makefile pins (ELFIO_VERSION: the last one without C++17).
        naxElfio = pkgs.fetchFromGitHub {
          owner = "serge1"; repo = "ELFIO";
          rev = "d251da09a07dff40af0b63b8f6c8ae71d2d1938d";
          hash = "sha256-0uqWs+W6ePBgol3IKLr5GMx7b6Fm3SsV3KuRMTZCjaA=";
        };
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
        # Vortex's and Caliptra's makefiles call the 32-bit tools riscv32-unknown-elf-*; Caliptra
        # also runs the preprocessor (cpp) and size.
        riscv32Aliases = pkgs.runCommand "riscv32-unknown-elf-aliases" { } ''
          mkdir -p $out/bin
          ln -s ${rv32.gcc}/bin/riscv32-none-elf-gcc $out/bin/riscv32-unknown-elf-gcc
          ln -s ${rv32.gcc}/bin/riscv32-none-elf-cpp $out/bin/riscv32-unknown-elf-cpp
          for tool in as ld nm objcopy objdump readelf size; do
            ln -s ${rv32.binutils}/bin/riscv32-none-elf-$tool $out/bin/riscv32-unknown-elf-$tool
          done
        '';
        # The nixpkgs 32 bit newlib is built for rv32imafdc and the ilp32d ABI. The VeeR EL2 core of Caliptra
        # has no FPU, and its firmware is built for rv32imc with ilp32, which cannot link to those libraries.
        newlibRv32imc = pkgs.pkgsCross.riscv32-embedded.newlib.overrideAttrs (o: {
          preConfigure = o.preConfigure + ''
            export CFLAGS_FOR_TARGET="-g -O2 -march=rv32imc_zicsr_zifencei -mabi=ilp32"
          '';
        });
        # The compiler of BSG Manycore: the vanilla core is RV32IMAF with the ilp32f ABI (software/mk/Makefile.builddefs:
        # ARCH_OP, ABI), which the 32 bit toolchain of nixpkgs (rv32imafdc, ilp32d, no multilib) cannot link. GCC, binutils
        # and newlib are built again with that default. zicsr and zifencei because binutils 2.46 does not take the CSR
        # instructions of crt.S in plain rv32imaf.
        rv32f = (import nixpkgs {
          inherit system;
          crossSystem = {
            config = "riscv32-none-elf";
            libc = "newlib";
            gcc = { arch = "rv32imaf_zicsr_zifencei"; abi = "ilp32f"; };
          };
        }).buildPackages;
        # The Makefiles of BSG Manycore call riscv32-unknown-elf-dramfs-* from RISCV_BIN_DIR. They pass -mtune=bsg_vanilla_2020,
        # a scheduling model of the patched GCC 9.2 that upstream builds; stock GCC does not know it, so the wrappers drop
        # that one option and nothing else.
        bsgCc = name: real: pkgs.writeShellScriptBin "riscv32-unknown-elf-dramfs-${name}" ''
          args=()
          for a in "$@"; do [ "$a" = -mtune=bsg_vanilla_2020 ] || args+=("$a"); done
          exec ${real} "''${args[@]}"
        '';
        bsgRiscv = pkgs.symlinkJoin {
          name = "bsg-riscv32-ilp32f";
          paths = [
            (bsgCc "gcc" "${rv32f.gcc}/bin/riscv32-none-elf-gcc")
            (bsgCc "g++" "${rv32f.gcc}/bin/riscv32-none-elf-g++")
            (pkgs.runCommand "bsg-riscv32-binutils" { } ''
              mkdir -p $out/bin
              for tool in objcopy objdump ar; do
                ln -s ${rv32f.binutils}/bin/riscv32-none-elf-$tool $out/bin/riscv32-unknown-elf-dramfs-$tool
              done
              ln -s ${rv32f.binutils}/bin/riscv32-none-elf-objdump $out/bin/riscv32-unknown-elf-objdump
            '')
          ];
        };
        # The Python of the cocotb tests of Taxi. Versions are those of the tox.ini of Taxi: cocotb 2.0.1, cocotb-bus 0.3.0,
        # cocotb-test 0.2.6, cocotbext-axi and cocotbext-eth 0.1.28, cocotbext-i2c 0.1.2, cocotbext-uart 0.1.4, scapy 2.6.1.
        # Python 3.13: the newest that cocotb 2.0.1 ships wheels for.
        cocotbPy =
          let
            py = pkgs.python313.pkgs;
            ext = { pname, version, hash, deps ? [ ] }: py.buildPythonPackage {
              inherit pname version;
              pyproject = true;
              src = pkgs.fetchPypi { inherit pname version hash; };
              build-system = [ py.setuptools py.setuptools-scm ];
              dependencies = deps;
              doCheck = false;
            };
            cocotb = py.cocotb.overridePythonAttrs (_: {
              version = "2.0.1";
              src = pkgs.fetchPypi {
                pname = "cocotb"; version = "2.0.1";
                hash = "sha256-aYh3SEEv9D6Y+Fea1sDaH2/xmpTQw7TW2kctjoZ4ToI=";
              };
              patches = [ ];
              doCheck = false;
            });
            scapy = py.scapy.overridePythonAttrs (_: {
              version = "2.6.1";
              src = pkgs.fetchPypi {
                pname = "scapy"; version = "2.6.1";
                hash = "sha256-dgDX4jg8hT5cOm4F034XZDvuvys+ENeRTf/MO8PG5sU=";
              };
              patches = [ ];
              doCheck = false;
            });
            cocotbBus = ext { pname = "cocotb_bus"; version = "0.3.0"; hash = "sha256-l2KyknP/Bi9SFg5XJ048sQbRTn53ZRXeE3LB1zVGsAU="; deps = [ cocotb scapy ]; };
            cocotbTest = ext { pname = "cocotb_test"; version = "0.2.6"; hash = "sha256-pGYZSMoUXu5rzK+DIXS177n200IGdxV8BK3nK7IWBpI="; deps = [ cocotb py.pytest py.find-libpython ]; };
            axi = ext { pname = "cocotbext_axi"; version = "0.1.28"; hash = "sha256-XQYhhbm7VHaDmh2BaCHEUzskX1b3sXGz85Ew4miRrjc="; deps = [ cocotb cocotbBus ]; };
            eth = ext { pname = "cocotbext_eth"; version = "0.1.28"; hash = "sha256-vBmx/vG0uSBfgoIrpFmRUZAAKS57iaZwFvWWwCLhBpU="; deps = [ cocotb cocotbBus axi ]; };
            i2c = ext { pname = "cocotbext_i2c"; version = "0.1.2"; hash = "sha256-GIxv+cyZmFjutQVgtBosevAEDzNKvvsJcfjEC/iBiGc="; deps = [ cocotb ]; };
            uart = ext { pname = "cocotbext_uart"; version = "0.1.4"; hash = "sha256-V0ZDyCBnvl4d1uLoMPRriAPAf8Hye391vyUGb9bQZ5g="; deps = [ cocotb ]; };
          in
          pkgs.python313.withPackages (_: [ cocotb cocotbBus cocotbTest axi eth i2c uart scapy py.pytest ]);
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
          # libelf, which the OpenTitan simulation links; zlib for the FST writer of Verilator 4 (NaxRiscv)
          buildInputs = [ pkgs.elfutils pkgs.zlib ];
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
            # Chipyard's and NaxRiscv's generation flows (JDK 17: the Scala 2.12 of sbt cannot read newer class files)
            jdk17
            sbt
            dtc
            perl
            gawk
            which
            wallyShims
            # the build.sh of gemmini-rocc-tests runs autoconf for its Makefile
            autoconf
            # Chipyard installs the espresso logic minimizer (a submodule of constellation) with cmake; the Chisel
            # decoder of Saturn runs out of memory without it
            cmake
          ];
          # uv runs upstream scripts (elf2hex) with the Python above, never downloads one.
          UV_PYTHON_DOWNLOADS = "never";
          RISCV_NEWLIB_MEDANY = "${newlibMedany}/riscv64-none-elf/lib";
          RISCV_NEWLIB_RV32IMC = "${newlibRv32imc}/riscv32-none-elf/lib";
          RISCV_EMPTY_CRT = "${emptyCrt}";
          VX_SOFTFLOAT = "${vxSoftfloat}";
          VX_RAMULATOR = "${vxRamulator}";
          # libfesvr and libriscv (Spike) for the Chipyard simulator
          CY_SPIKE = "${pkgs.spike}";
          NAX_VERILATOR4 = "${verilator4}";
          # cocotb-config and the Python of the cocotb tests (Taxi)
          COCOTB_ENV = "${cocotbPy}";
          NAX_SPIKE = "${naxSpike}";
          NAX_ELFIO = "${naxElfio}";
          NAX_SDL2 = "${pkgs.SDL2.dev}";
          NAX_SDL2_LIB = "${pkgs.SDL2}/lib";
        };
        # `nix develop .#bsg`: the shell that builds the workloads of BSG Manycore. Its compiler is built from source (GCC and
        # newlib, about an hour), so the default shell does not have it. The Makefiles of BSG Manycore compute with bc.
        devShells.bsg = pkgs.mkShell {
          inputsFrom = [ self.devShells.${system}.default ];
          packages = [ pkgs.bc ];
          # RISCV_BIN_DIR of its makefiles
          BSG_RISCV_BIN = "${bsgRiscv}/bin";
        };
      });
}
