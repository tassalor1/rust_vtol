{
  description = "Rust MAVLink VTOL Companion Computer DevShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "rust-vtol";

          buildInputs = with pkgs; [
            rustup
            cargo
            protobuf
            clang
            clang-tools  # gives you clangd, clang-format
            git
            pkg-config
            openssl.dev
          ];

          shellHook = ''
            echo "Rust MAVLink VTOL DevShell ready"
          '';
        };
      });
}
