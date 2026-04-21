{
  self,
  inputs,
  lib,
  pkgs,
  ...
}: {
  flake = {
    overlays.default = final: prev: {
      papertimed = self.packages.${final.system}.papertimed;
    };

    homeModules.default = {
      config,
      lib,
      system,
      pkgs,
      ...
    }: {
      options.services.papertimed = {
        enable = lib.mkEnableOption "papertimed";

        package = lib.mkOption {
          type = lib.types.package;
          default = self.packages.${system}.papertimed;
          description = "Package used by papertimed service.";
        };

        settings = lib.mkOption {
          type = lib.types.attrs;
          default = {};
          description = "Papertimed configuration";
          example = {
            global = {
              adapter = "hyprpaper";
            };

            schedules = [
              {
                id = "school";
                rules = [
                  {
                    week_days = ["mon" "tue" "wed" "thu" "fri"];
                  }
                  {
                    day_time = {
                      from = "08:00";
                      to = "17:00";
                    };
                  }
                ];
              }
              {
                id = "main";
                rules = [
                  {
                    day_time = {
                      from = "00:00";
                      to = "17:00";
                    };
                  }
                ];
              }
            ];

            wallpapers = [
              {
                filename = "someimage.jpg";
                monitors = ["HDMI-A-1" "DP-1"];
                schedules = ["school"];
              }
              {
                filename = "otherimage.jpg";
                monitors = ["eDP-1"];
                schedules = ["main"];
              }
            ];
          };
        };
      };

      config = let
        yamlFormat = pkgs.formats.yaml {};
      in
        lib.mkIf config.services.papertimed.enable {
          systemd.user.services.papertimed = {
            Unit = {
              Description = "Papertimed wallpaper service";
            };
            Service = {
              Type = "simple";
              ExecStart = "${lib.getExe config.services.papertimed.package}";
              Restart = "no";
            };
            Install = {
              WantedBy = ["default.target"];
            };
          };

          home.packages = [
            self.packages.${system}.papertimed
          ];

          home.file.".config/papertimed/config.yaml" = {
            enable = true;
            source =
              yamlFormat.generate "config.yaml" config.services.papertimed.settings;
          };
        };
    };
  };
}
