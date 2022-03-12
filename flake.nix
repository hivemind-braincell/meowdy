{
  description = "meowdy";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nmattia/naersk";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , naersk
    } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };

      rust-toolchain =
        (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
          extensions = [ "rust-src" ];
        };

      nightly-rustfmt = pkgs.rust-bin.nightly.latest.rustfmt;

      naersk-lib = naersk.lib."${system}".override {
        rustc = rust-toolchain;
      };

      format-pkgs = with pkgs; [
        nixpkgs-fmt
      ];
    in
    rec
    {
      packages.meowdy = naersk-lib.buildPackage {
        pname = "meowdy";
        root = ./.;
        nativeBuildInputs = with pkgs; [ ];
      };
      defaultPackage = packages.meowdy;

      apps.meowdy = flake-utils.lib.mkApp {
        drv = packages.meowdy;
      };
      defaultApp = apps.meowdy;

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          # The ordering of these two items is important. For nightly rustfmt to be used instead of 
          # the rustfmt provided by `rust-toolchain`, it must appear first in the list. This is 
          # because native build inputs are added to $PATH in the order they're listed here.
          nightly-rustfmt
          rust-toolchain

          pkgconfig
        ] ++ format-pkgs;
        buildInputs = with pkgs; [
          dbus
          udev alsaLib vulkan-loader
          xlibsWrapper xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libxcb
        ];
        shellHook = with pkgs; ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath [
          dbus
          udev alsaLib vulkan-loader xorg.libxcb
        ]}"'';
      };

      checks = {
        format = pkgs.runCommand
          "check-nix-format"
          { buildInputs = format-pkgs; }
          ''
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out
          '';
      };
    });
}
