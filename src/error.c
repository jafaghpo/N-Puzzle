/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 18:12:29 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/04 17:16:09 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

void	exit_program(char *error_message)
{
	printf("%s\n", error_message);
	exit(EXIT_FAILURE);
}
