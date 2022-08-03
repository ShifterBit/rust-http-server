{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    naersk-lib = pkgs.callPackage naersk {};
  in {
    packages.x86_64-linux.default = naersk-lib.buildPackage ./.;

    apps.x86_64-linux.default = {
      type = "app";
      program = "${self.packages.x86_64-linux.default}/bin/rust-http-server";
    };

    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [cargo rustc rustfmt pre-commit rustPackages.clippy];
      RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
    };
  };
}
