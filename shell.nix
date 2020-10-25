with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup

    # Example Build-time Additional Dependencies
    # pkg-config
    # systemd.dev
    # alsaLib
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    # openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}