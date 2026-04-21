{
  lib,
  inputs,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.easyOverlay];

  perSystem = {
    pkgs,
    self',
    config,
    ...
  }: {
    overlayAttrs = {
      inherit (config.packages) papertimed;
    };

    packages.default = self'.packages.papertimed;

    packages.papertimed = let
      cargoToml = fromTOML (builtins.readFile ../Cargo.toml);
    in
      pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;

        src = ../.;

        cargoLock = {
          lockFile = ../Cargo.lock;
        };

        meta = {
          description = "Papaertimed is a little daemon to controll which wallpaper is
    shown at what time.";
          license = lib.licenses.mit;
        };
      };
  };
}
