{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    wgsl-analyzer.url = "github:wgsl-analyzer/wgsl-analyzer";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        config,
        pkgs,
        system,
        lib,
        self',
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        formatter = pkgs.alejandra;

        packages = {
          stable-rust-toolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = ["rust-src" "clippy"];
            targets = ["wasm32-unknown-unknown"];
          };

          nightly-rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-src" "clippy"];
              targets = ["wasm32-unknown-unknown"];
            });

          nightly-rustfmt = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rustfmt"];
            });

          nightly-rust-analyzer = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-analyzer"];
            });

          # Wrap cargo udeps to use the nightly toolchain
          cargo-udeps = pkgs.stdenv.mkDerivation {
            name = "cargo-udeps";
            buildInputs = [pkgs.makeWrapper];
            buildCommand = ''
              mkdir -p $out/bin
              ln -s ${pkgs.cargo-udeps}/bin/cargo-udeps $out/bin/cargo-udeps-unwrapped
              wrapProgram $out/bin/cargo-udeps-unwrapped \
                --prefix PATH ":" "${config.packages.nightly-rust-toolchain}/bin"
              mv $out/bin/cargo-udeps-unwrapped $out/bin/cargo-udeps
            '';
          };
        };

        devShells.default = let
          buildInputs = [
            pkgs.pkg-config

            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libxcb
            pkgs.xorg.libXi
            pkgs.xorg.libXft
            pkgs.xorg.libXrandr

            pkgs.wayland
            pkgs.libxkbcommon

            pkgs.shaderc
            pkgs.directx-shader-compiler
            pkgs.libGL
            pkgs.vulkan-headers
            pkgs.vulkan-loader
            pkgs.vulkan-tools
            pkgs.vulkan-tools-lunarg
            pkgs.vulkan-validation-layers
          ];
        in pkgs.mkShell {
          inherit buildInputs;
          packages = [
            # Building
            config.packages.stable-rust-toolchain
            pkgs.cmake
            pkgs.gnumake
            pkgs.pkg-config

            # Dev tools
            inputs.wgsl-analyzer.packages.${system}.default
            config.formatter
            config.packages.nightly-rust-analyzer
            config.packages.nightly-rustfmt
            config.packages.cargo-udeps
            pkgs.taplo
            pkgs.just
            pkgs.fd
            pkgs.watchexec
            pkgs.cargo-nextest
            pkgs.cargo-outdated
            pkgs.cargo-audit
            pkgs.nodePackages.prettier
            pkgs.websocat
            pkgs.trunk
            pkgs.mdbook

            # Pull in dependencies that Trunk would otherwise download at runtime.
            # Trunk pulls in precompiled binaries that are incompatible with NixOS.
            # Therefore provide them through the devshell.
            # Look out for anything Trunk pulls in at runtime. Trunk places stuff into ~/.cache/trunk/
            pkgs.dart-sass
            pkgs.wasm-bindgen-cli
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      };
    };
}
