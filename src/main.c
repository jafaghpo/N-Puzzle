/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 13:20:36 by ggregoir          #+#    #+#             */
/*   Updated: 2019/02/20 11:14:17 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

int	main(int argc, char **argv)
{
	char *file;
	t_node *initial_state;
	t_info info;

	printf("argc = %d\n", argc);
	file = ft_filetostr(argv[1]);
	initial_state = parse_file(file, &info);
	//goal = generate_goal(info)    t_info  
	// a * solver
	free(file);
	return 0;
}