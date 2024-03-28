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

        devShells.default = pkgs.mkShell {
          packages = [
            # Building
            config.packages.stable-rust-toolchain
            pkgs.cmake
            pkgs.gnumake
            pkgs.pkg-config

            # OBS: Updated to a much more correct way of exposing these libs from the system nixos/home-manager conf
            # Simulation runtime deps
            # pkgs.fontconfig
            # pkgs.freetype
            # pkgs.libxkbcommon # (Prevents runtime panic in winit)
            # pkgs.wayland
            # pkgs.vulkan-loader
            # pkgs.vulkan-validation-layers

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

          # winit makes assumptions about what shared libraries are available at runtime, or nix fails to recognize which transient shared libraries will be loaded.
          # In a real nix derivation this would go in buildinputs, but for the shell here we overwrite LD_LIBRARY_PATH.
          #
          # winit uses dlopen or specifically wayland-dlopen.
          #
          # OBS: `pkgs` here must match the nixpkgs revision for the system (the last time I did nixos-rebuild switch)!
          # Otherwise, if it's not in sync winit errors at runtime. Therefore, write a dummy package in the system nixos or home manager configuration to expose these libs
          LD_LIBRARY_PATH = "/home/plul/.local/state/nix/profiles/home-manager/home-path/lib";
          # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          #   pkgs.libxkbcommon
          #   pkgs.wayland
          #   pkgs.vulkan-loader
          # ];
        };
      };
    };
}
