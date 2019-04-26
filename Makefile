NAME = npuzzle

red="\033[31m"
green="\033[32m"
blue="\033[34m"
reset="\033[0m"

all: build_release $(NAME)

build_release:
	@cargo build --release

$(NAME):
	@echo $(blue)"Linking release executable..."$(reset)
	@ln -s target/release/npuzzle
	@echo $(green)"Linking done"$(reset)

build_debug:
	@cargo build

debug: build_debug
	@echo $(red)"Deleting existing link to executable..."$(reset)
	@rm npuzzle
	@echo $(blue)"Linking debug executable..."$(reset)
	@ln -s target/debug/npuzzle
	@echo $(green)"Linking done"$(reset)

clean: 
	@rm -rf npuzzle
	@echo $(red)"Deleted symbolic link of executable"$(reset)
	@rm -rf target
	@echo $(red)"Deleting target directory"$(reset)

re: clean all
.PHONY: all clean re cargo
