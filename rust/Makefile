help:
	@echo "Makefile for git-wyag"

# Create git wrapper
setup:
	@cargo check
	@chmod +x env/git
	@chmod +x env/make

check:
	@cargo check

test: setup
	@cargo test

# Run in subshell with override
run: setup
	@echo "Starting new shell with git overridden by ./env/git"
	@echo "Type 'exit' to return to your original shell"
	@PATH="$(PWD)/env:$$PATH" PS1='\[\e[1;35m\](git-wyag)\[\e[0m\] $$ ' bash --norc --noprofile -i

-include Makefile.private
makefile.private: ;