/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 17:12:30 by ggregoir          #+#    #+#             */
/*   Updated: 2019/02/22 18:14:00 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

static int get_grid_size(char *file, int *cursor)
{
	int result;
	while (!(isdigit(file[*cursor])))
	{
		if (file[*cursor] != '#' && !isspace(file[*cursor]))
			exit_program("bad input");

		if (file[*cursor] == '#')
		{
			while (file[*cursor] != '\n')
			{
				*cursor+=1;
			}
		}
		*cursor+=1;
	}
	result = atoi(file + *cursor);
	while(isdigit(file[*cursor]))
		*cursor+=1;
	while (file[*cursor] != '\n')
	{
		if (file[*cursor] != '#' && !isspace(file[*cursor]))
			exit_program("bad input");
		if (file[*cursor] == '#')
		{
			while (file[*cursor] != '\n')
			{
				*cursor+=1;
			}
			return(result);
		}
		*cursor+=1;
	}
	return(result);
}



static void get_initial_state(char *file, int *cursor, t_info *info, t_node *initial_state)
{
	int lines = 0;
	int columns = 0;
	while (columns != info->grid_size)
	{
		while (lines != info->grid_size)
		{
			while (!(isdigit(file[*cursor])))
			{
				if (file[*cursor] != '#' && !isspace(file[*cursor]))
					exit_program("bad input");

				if (file[*cursor] == '#')
				{
					while (file[*cursor] != '\n')
					{
						*cursor+=1;
					}
				}
				*cursor+=1;
			}
			printf("file[cursor] pre atoi = %c\n", file[*cursor]);
			initial_state->map[columns][lines] = atoi(file + *cursor);
			while(isdigit(file[*cursor]))
				*cursor+=1;
			lines++;
		}
		printf("file[cursor] post atoi = %c\n", file[*cursor]);
		while (file[*cursor] != '\n')
		{
			printf("file[cursor] while = %c\n", file[*cursor]);
			if (file[*cursor] != '#' && !isspace(file[*cursor]))
				exit_program("bad input");
			if (file[*cursor] == '#')
			{
				while (file[*cursor] != '\n')
				{
					*cursor+=1;
				}
			}
			*cursor+=1;
		}
		columns++;
		lines = 0;
	}
}

void	print_map(t_node *initial_state, t_info *info)
{
	int x = 0;
	int y = 0;

	printf("\nMAP :\n");

	while (y != info->grid_size)
	{
		while (x != info->grid_size)
		{
			printf("%d ", initial_state->map[y][x]);
			x++;
		}
		printf("\n");
		x = 0;
		y++;
	}
}

t_node *parse_file(char *file, t_info *info)
{
	int cursor = 0;
	int x = 0;
	t_node *initial_state = NULL;

	info->grid_size = get_grid_size(file, &cursor);

	if ((initial_state = malloc(sizeof(t_node))) == NULL)
		exit_program("malloc error in parse file");
	if ((initial_state->map = malloc(sizeof(int*) * info->grid_size)) == NULL)
		exit_program("malloc error in parse file");
	while (x != info->grid_size)
		if ((initial_state->map[x++] = malloc(sizeof(int) * info->grid_size)) == NULL)
			exit_program("malloc error in parse file");

	get_initial_state(file, &cursor, info, initial_state);
	print_map(initial_state, info);
	return (initial_state);
}