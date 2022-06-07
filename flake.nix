{
  description = "A very basic flake";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/release-21.11;

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    libs = (with pkgs; [libGL pkgconfig]) ++ (with pkgs.xorg; [libX11 libXcursor libXrandr libXi]);
  in {
    defaultPackage.${system} = pkgs.hello;
    devShell.${system} = pkgs.mkShell {
      nativeBuildInputs = libs;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
    };

  };
}
