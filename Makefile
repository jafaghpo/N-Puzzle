# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/02/19 12:38:42 by ggregoir          #+#    #+#              #
#    Updated: 2019/02/20 11:14:57 by ggregoir         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME			=	npuzzle

CC				=	gcc
PATH_PROJECT	=	$(shell pwd)
CFLAGS			=	-Wall -Werror -Wextra -g -D PATH=\"$(PATH_PROJECT)\"

ifeq ($(DEBUG), yes)
	CFLAGS	+=	-g -D DEBUG -fsanitize=address
else
	CFLAGS	+=	-Ofast
endif

SRCS		=	main.c file.c parse.c error.c
SRC			=	$(addprefix src/, $(SRCS))
OBJ			=	$(addprefix obj/, $(SRCS:.c=.o))
INCL_DIR	=	include
INCL		=	$(INCL_DIR)/npuzzle.h


all: $(NAME)

$(NAME): $(OBJ)
	$(CC) -o $@ $(OBJ) $(CFLAGS)

obj/%.o: src/%.c $(INCL)
	$(CC) -o $@ -c $< -I $(INCL_DIR) $(CFLAGS)

clean:
	rm -f $(OBJ)

fclean: clean
	rm -f $(NAME)

re: fclean all