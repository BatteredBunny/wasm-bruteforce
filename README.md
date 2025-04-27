<h1 align="center">WASM bruteforce thingymajig</h1>
<p align="center">Bruteforce images from image hosting website</p>

https://github.com/BatteredBunny/wasm-bruteforce/assets/52951851/6b3c6b7e-6389-4ed0-99fd-94db986e38a3

## Start in development mode
```sh
nix develop
make start
```

## Usage with caddy on nixos


```nix
# flake.nix
inputs = {
    wasm-bruteforce.url = "github:BatteredBunny/wasm-bruteforce";
};
```

```nix
# configuration.nix
nixpkgs.overlays = [
  inputs.wasm-bruteforce.overlays.default
];

caddy.virtualHosts."brute.example.com".extraConfig = ''
  root * ${pkgs.wasm-bruteforce}
  file_server
'';
```
