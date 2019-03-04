/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 13:20:36 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/04 17:23:56 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

int	main(int argc, char **argv)
{
	char *file;
	t_node *initial_state;
	t_info info;

	info = parse_args(int argc, char **argv);
	file = ft_filetostr(argv[1]);
	initial_state = parse_file(file, &info);
	free(file);
	print_map(initial_state, info);
	//goal = generate_goal(info)    t_info  
	// a * solver
	return 0;
}
