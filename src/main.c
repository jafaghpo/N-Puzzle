/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 13:20:36 by ggregoir          #+#    #+#             */
/*   Updated: 2019/02/19 16:37:15 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

int	main(int argc, char **argv)
{
	char *str;

	printf("argc = %d\n", argc);
	str = ft_filetostr(argv[1]);
	free(str);
	return 0;
}