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

    packages.papertimed =
      pkgs.rustPlatform.buildRustPackage {
        pname = "papertimed";
        version = "0.1.0";

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
