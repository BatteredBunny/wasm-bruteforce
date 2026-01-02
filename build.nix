{
  stdenv,
  rustPlatform,
  openssl,
  gnumake,
  pkg-config,
  llvmPackages,
  wasm-bindgen-cli_0_2_106,

  fetchPnpmDeps,
  nodejs,
  pnpmConfigHook,
  pnpm_10,
}:
let
  targetName = "wasm32-unknown-unknown";
  pname = "wasm-bruteforce";
  version = "0.4.5";

  wasm-build = rustPlatform.buildRustPackage {
    inherit pname version;

    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = [
      wasm-bindgen-cli_0_2_106
      pkg-config
      llvmPackages.lld
    ];

    buildInputs = [
      openssl
      gnumake
    ];

    doCheck = false;

    buildPhase = ''
      runHook preBuild

      cargo build --target ${targetName} --release

      mkdir -p $out/pkg
      wasm-bindgen target/${targetName}/release/wasm_bruteforce.wasm --out-dir=$out/pkg

      runHook postBuild
    '';

    installPhase = "echo 'Skipping installPhase'";
  };
in
stdenv.mkDerivation (finalAttrs: {
  inherit pname version;

  src = ./www;

  nativeBuildInputs = [
    nodejs
    pnpmConfigHook
    pnpm_10
  ];

  buildPhase = ''
    runHook preBuild

    ln -s ${wasm-build}/pkg ../pkg
    pnpm build
    cp -r dist $out

    runHook postBuild
  '';

  pnpmDeps = fetchPnpmDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 2;
    hash = "sha256-PtJohmqwFlWnx2vHjbf6QI8efjxnoVANUxRSo2EcdKk=";
  };
})
