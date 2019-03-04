/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse_file.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 17:12:30 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/04 17:52:42 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

static int		get_grid_size(char *file, int *cursor)
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
				*cursor += 1;
			}
		}
		*cursor += 1;
	}
	result = atoi(file + *cursor);
	while (isdigit(file[*cursor]))
		*cursor += 1;
	while (file[*cursor] != '\n')
	{
		if (file[*cursor] != '#' && !isspace(file[*cursor]))
			exit_program("bad input");
		if (file[*cursor] == '#')
		{
			while (file[*cursor] != '\n')
			{
				*cursor += 1;
			}
			return (result);
		}
		*cursor += 1;
	}
	*cursor += 1;
	return (result);
}

static t_node	*get_initial_state(char *file, int *cursor, t_info *info)
{
	int		lines;
	int		columns;
	int		x;
	t_node	*result;

	lines = 0;
	columns = 0;
	x = 0;
	*cursor += 1;
	if ((result = malloc(sizeof(t_node))) == NULL)
		exit_program("malloc error in parse file");
	if ((result->map = malloc(sizeof(int*) * info->grid_size)) == NULL)
		exit_program("malloc error in parse file");
	while (x != info->grid_size)
		if ((result->map[x++] = malloc(sizeof(int) * info->grid_size)) == NULL)
			exit_program("malloc error in parse file");
	while (columns != info->grid_size)
	{
		while (lines != info->grid_size)
		{
			while (!(isdigit(file[*cursor])))
			{
				if (file[*cursor] == '\n')
					exit_program("bad imput");
				if (file[*cursor] != '#' && !isspace(file[*cursor]))
					exit_program("bad input");
				if (file[*cursor] == '#')
				{
					while (file[*cursor] != '\n')
					{
						*cursor += 1;
					}
				}
				*cursor += 1;
			}
			result->map[columns][lines] = atoi(file + *cursor);
			while (isdigit(file[*cursor]))
				*cursor += 1;
			lines++;
		}
		while (file[*cursor] != '\n')
		{
			if (file[*cursor] != '#' && !isspace(file[*cursor]))
				exit_program("bad input");
			if (file[*cursor] == '#')
			{
				while (file[*cursor] != '\n')
				{
					*cursor += 1;
				}
			}
			else
				*cursor += 1;
		}
		*cursor += 1;
		columns++;
		lines = 0;
	}
	while (file[*cursor])
	{
		if (file[*cursor] == '#')
		{
			while (file[*cursor] != '\n')
			{
				*cursor += 1;
			}
		}
		else if (isspace(file[*cursor]))
			*cursor += 1;
		else
			exit_program("bad input");
	}
	return (result);
}

void			print_map(t_node *initial_state, t_info info)
{
	int x;
	int y;

	x = 0;
	y = 0;
	printf("\nMAP :\n");
	while (y != info.grid_size)
	{
		while (x != info.grid_size)
		{
			printf("%d ", initial_state->map[y][x]);
			x++;
		}
		printf("\n");
		x = 0;
		y++;
	}
}

t_node			*parse_file(char *file, t_info *info)
{
	int		cursor;
	t_node	*initial_state;

	cursor = 0;
	*initial_state = NULL;
	info->grid_size = get_grid_size(file, &cursor);
	initial_state = get_initial_state(file, &cursor, info);
	return (initial_state);
}
