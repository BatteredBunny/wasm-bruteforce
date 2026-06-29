{
  stdenv,
  rustPlatform,
  openssl,
  pkg-config,
  llvmPackages,
  wasm-bindgen-cli_0_2_121,

  fetchPnpmDeps,
  nodejs,
  pnpmConfigHook,
  pnpm_11,
}:
let
  targetName = "wasm32-unknown-unknown";
  pname = "wasm-bruteforce";
  version = "0.4.5";
  pnpm = pnpm_11;

  wasm-build = rustPlatform.buildRustPackage {
    inherit pname version;

    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = [
      wasm-bindgen-cli_0_2_121
      pkg-config
      llvmPackages.lld
    ];

    buildInputs = [
      openssl
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
    pnpm
  ];

  buildPhase = ''
    runHook preBuild

    ln -s ${wasm-build}/pkg ../pkg
    pnpm build
    cp -r dist $out

    runHook postBuild
  '';

  pnpmDeps = fetchPnpmDeps {
    pname = "wasm-bruteforce-frontend";
    inherit (finalAttrs) version src;
    inherit pnpm;
    fetcherVersion = 3;
    hash = "sha256-UyFhEOACzNz7lYjEKqmzfFzaxi+O32BZLzh5oUnQvaM=";
  };
})
