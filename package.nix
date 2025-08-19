{
  lib,
  rustPlatform,
  pkg-config,
  alsa-lib,
  libxkbcommon,
  stdenv,
  vulkan-loader,
  xorg,
  udev,
}:
rustPlatform.buildRustPackage rec {
  pname = "tcss445project";
  version = "0.1.0";

  src = ./.;

  useFetchCargoVendor = true;
  cargoHash = "sha256-Ov2E2H/mZzjJV6U/v/RpRyS1wnDvkR5pt5VCB4JL9pw=";

  buildNoDefaultFeatures = true;
  buildFeatures = [];

  nativeBuildInputs = [pkg-config];

  buildInputs =
    lib.optionals (stdenv.hostPlatform.isLinux) [
      # for Linux
      # Audio (Linux only)
      alsa-lib
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      udev
      vulkan-loader
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      rustPlatform.bindgenHook
    ];

  postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
    patchelf $out/bin/tcss445project \
      --add-rpath ${lib.makeLibraryPath buildInputs}
  '';

  meta = {
    description = "A School project that happens to be a game.";
    homepage = "https://github.com/elijahimmer/tcss445project";
    changelog = "https://github.com/elijahimmer/tcss445project/blob/v${version}/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = "tcss445project";
  };
}
