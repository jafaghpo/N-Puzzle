/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   npuzzle.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 12:55:02 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/02 19:37:51 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef NPUZZLE_H
# define NPUZZLE_H

# include <fcntl.h>
# include <unistd.h>
# include <string.h>
# include <stdio.h>
# include <stdint.h>
# include <stdlib.h>
# include <sys/stat.h>
# include <ctype.h>

# define MAX_FILE_SIZE 209715200


typedef enum 	e_heuristic	t_heuristic;
typedef enum 	e_algo		t_algo;
typedef struct	s_node		t_node;
typedef struct	s_info		t_info;

struct s_node
{
	t_node	*parent;
	int32_t	**map;
	uint32_t	f;
	uint32_t	h;
	uint32_t	g;
};

struct s_info
{
	int32_t	grid_size;
	
};

enum e_heuristic
{
	 MANHATAN,
	 HAMMING,
	 CONFLICT
};

enum e_algo
{
	 ASTAR,
	 BFS,
	 DFS,
	 GREEDY,
	 UCS
};


char	*ft_filetostr(char *file);
t_node *parse_file(char *file, t_info *info);
void	exit_program(char *error_message);
void	print_map(t_node *initial_state, t_info info);

#endif