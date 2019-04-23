NAME = npuzzle

red="\033[31m"
blue="\033[34m"
reset="\033[0m"

all: cargo $(NAME)

cargo:
	@cargo build --release

$(NAME):
	@echo $(blue)"Linking..."$(reset)
	@ln -s target/release/npuzzle

clean: 
	@rm -rf npuzzle
	@echo $(red)"Deleted symbolic link of executable"$(reset)
	@rm -rf target
	@echo $(red)"Deleting target directory"$(reset)

re: clean all
.PHONY: all clean re cargo
