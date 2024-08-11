{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    basecamp.url = "github:plul/basecamp/refactor";
    basecamp.inputs.nixpkgs.follows = "nixpkgs";
    basecamp.inputs.rust-overlay.follows = "rust-overlay";

    flake-parts.url = "github:hercules-ci/flake-parts";
    wgsl-analyzer.url = "github:wgsl-analyzer/wgsl-analyzer";
  };

  outputs =
    inputs@{ basecamp, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem =
        { pkgs, system, ... }:
        {
          devShells.default = basecamp.mkShell pkgs {
            rust.enable = true;
            rust.toolchain.targets = [ "wasm32-unknown-unknown" ];

            packages = [
              pkgs.cmake
              pkgs.gnumake
              pkgs.pkg-config

              inputs.wgsl-analyzer.packages.${system}.default
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

            env = {
              # winit makes assumptions about what shared libraries are available at runtime, or nix fails to recognize which transient shared libraries will be loaded.
              # In a real nix derivation this would go in buildinputs, but for the shell here we overwrite LD_LIBRARY_PATH.
              #
              # winit uses dlopen.
              #
              # Simulation runtime deps:
              # pkgs.fontconfig
              # pkgs.freetype
              # pkgs.libxkbcommon # (Prevents runtime panic in winit)
              # pkgs.wayland
              # pkgs.vulkan-loader
              # pkgs.vulkan-validation-layers
              # 
              # OBS: On NixOS, `pkgs` in the above must match the nixpkgs revision for the system (the revision of nixpkgs for the latest nixos-rebuild switch)!
              # Otherwise, if it's not in sync, then winit errors at runtime. 
              # Therefore, instead of setting LD_LIBRARY_PATH by 
              # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ (...) ]
              # instead, write a dummy package in the system nixos or home manager configuration to expose these libs for a revision that matches the system:
              LD_LIBRARY_PATH = "/home/plul/.local/state/nix/profiles/home-manager/home-path/lib";
            };
          };
        };
    };
}
