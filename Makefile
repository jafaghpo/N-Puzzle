NAME = npuzzle

all: build_release $(NAME)

build_release:
	@cargo build --release

$(NAME):
	@ln -s target/release/npuzzle

build_debug:
	@cargo build

debug: build_debug
	@rm npuzzle
	@ln -s target/debug/npuzzle

test:
	@cargo test

clean: 
	@rm -rf npuzzle
	@rm -rf target

re: clean all
.PHONY: all clean re cargo
