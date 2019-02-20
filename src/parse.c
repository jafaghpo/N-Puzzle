/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 17:12:30 by ggregoir          #+#    #+#             */
/*   Updated: 2019/02/20 15:48:27 by ggregoir         ###   ########.fr       */
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



static int **get_initial_state(char *file, int *cursor, t_info *info)
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

}

t_node *parse_file(char *file, t_info *info)
{
	int cursor = 0;
	t_node *initial_state;

	info->grid_size = get_grid_size(file, &cursor);
	initial_state->map = get_initial_state(file, &cursor, info);

	if ((initial_state->map = ft_memalloc(sizeof(int*) * info->grid_size)) == NULL)
		return (exit_program("malloc error in parse file"));
	while (x != info->grid_size)
		if ((initial_state->map[x++] = ft_memalloc(sizeof(int) * info->grid_size)) == NULL)
			return (exit_program("malloc error in parse file"));

	printf("cursor = %d grid_size = %d\n", cursor, info->grid_size);
	return (initial_state);
}