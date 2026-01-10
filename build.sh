meson setup _build --prefix="$PWD/_build/install" --reconfigure
ninja -C _build
ninja -C _build install
GSETTINGS_SCHEMA_DIR=_build/data ./_build/src/autopilot
