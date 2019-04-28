NAME = npuzzle

red="\033[31m"
green="\033[32m"
blue="\033[34m"
reset="\033[0m"

all: build_release $(NAME)

build_release:
	cargo build --release

$(NAME):
	ln -s target/release/npuzzle

build_debug:
	cargo build

debug: build_debug
	rm npuzzle
	ln -s target/debug/npuzzle

test:
	cargo test

clean: 
	rm -rf npuzzle
	rm -rf target

re: clean all
.PHONY: all clean re cargo
