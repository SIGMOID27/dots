{ pkgs }: {
	deps = [
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
        pkgs.cmake
        pkgs.sdl2
	];
}