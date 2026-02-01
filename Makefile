NAME := computorv1

.PHONY: all clean fclean re test

all $(NAME):
	cargo build --release
	cp ./target/release/$(NAME) .

clean fclean:
	cargo clean
	rm -f ./$(NAME)

re: fclean all

test:
	cargo test