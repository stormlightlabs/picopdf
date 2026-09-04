{
  description = "picopdf development environment and packages";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      pkgsFor = system: import nixpkgs {
        inherit system;
        config.allowUnfreePredicate = package:
          nixpkgs.lib.getName package == "saxonche";
      };
    in {
      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
          rustSource = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              let name = baseNameOf path;
              in name != ".git" && name != ".venv" && name != "target";
          };
          picopdf = pkgs.rustPlatform.buildRustPackage {
            pname = "picopdf";
            version = "0.1.0";
            src = rustSource;
            cargoLock.lockFile = ./Cargo.lock;
          };
          picopdfDocling = pkgs.python313Packages.buildPythonApplication {
            pname = "picopdf-docling";
            version = "0.1.0";
            pyproject = true;
            src = ./py/picopdf-docling;

            build-system = [ pkgs.python313Packages.hatchling ];
            dependencies = [ pkgs.python313Packages.docling ];
            nativeCheckInputs = [ pkgs.python313Packages.pytestCheckHook ];
            pythonImportsCheck = [ "picopdf_docling" ];

            meta = {
              description = "Internal Docling sidecar for picopdf";
              license = pkgs.lib.licenses.mit;
              mainProgram = "picopdf-docling";
            };
          };
        in {
          default = picopdf;
          inherit picopdf;
          picopdf-docling = picopdfDocling;
        });

      checks = forAllSystems (system:
        let
          pkgs = pkgsFor system;
          packages = self.packages.${system};
          rustSource = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              let name = baseNameOf path;
              in name != ".git" && name != ".venv" && name != "target";
          };
          cargoCheck = name: command: pkgs.rustPlatform.buildRustPackage {
            pname = "picopdf-${name}";
            version = "0.1.0";
            src = rustSource;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.clippy pkgs.rustfmt ];
            dontUseCargoBuild = true;
            doCheck = false;
            buildPhase = ''
              runHook preBuild
              ${command}
              runHook postBuild
            '';
            installPhase = ''
              mkdir -p $out
            '';
          };
        in {
          rust-format = cargoCheck "format" "cargo fmt --all -- --check";
          rust-tests = packages.picopdf;
          rust-clippy = cargoCheck "clippy" "cargo clippy --workspace --all-targets --offline -- -D warnings";
          python-tests = packages.picopdf-docling;
          sidecar-smoke = pkgs.runCommand "picopdf-docling-smoke" {
            nativeBuildInputs = [ packages.picopdf-docling ];
          } ''
            test "$(picopdf-docling --protocol-version)" = 1
            touch $out
          '';
        });

      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor system;
          rustShell = pkgs.mkShell {
            packages = with pkgs; [ cargo clippy rustc rustfmt ];
          };
        in {
          default = rustShell;
          rust = rustShell;
          python = pkgs.mkShell {
            packages = [ self.packages.${system}.picopdf-docling pkgs.uv ];
            env.UV_PYTHON_DOWNLOADS = "never";
          };
        });
    };
}
