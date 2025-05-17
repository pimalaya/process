{
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

pimalaya.mkShell (
  removeAttrs args [ "pimalaya" ]
  // {
    extraBuildInputs = "nixd,nixfmt-rfc-style,git-cliff";
  }
)
