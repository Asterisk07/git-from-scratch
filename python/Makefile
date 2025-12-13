# Create git wrapper
setup:
	@chmod +x git

# Run in subshell with override
run: setup
	@echo "Starting new shell with git overridden by ./git"
	@echo "Type 'exit' to return to your original shell"
	@PATH="$(PWD):$$PATH" PS1='\[\e[1;35m\](git-wyag)\[\e[0m\] $$ ' bash --norc --noprofile -i # with custom prompt + colors
	@# @PATH="$(PWD):$$PATH" PS1='(custom git) $$ ' bash --norc --noprofile -i 				   # with custom prompt
	@# @PATH="$(PWD):$$PATH" bash --norc --noprofile -i										   # basic