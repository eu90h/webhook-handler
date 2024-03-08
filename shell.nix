let
  pkgs = import <nixpkgs> {
    overlays = [
      (import <rust-overlay>)
    ];
  };
in
with pkgs;
mkShell {
  nativeBuildInputs = [
    rust-bin.beta.latest.default
  ];
}
