_default:
	@just --list

run game:
	nom build -L --keep-failed {{justfile_directory()}}#{{game}}
	nix run {{justfile_directory()}}#{{game}}
