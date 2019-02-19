/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   file.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 13:01:14 by ggregoir          #+#    #+#             */
/*   Updated: 2019/02/19 16:36:35 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"


char	*ft_filetostr(char *file)
{
	int		fd;
	char	*str;
	struct stat st;
	size_t size;

	str = NULL;
	if ((fd = open(file, O_RDONLY)) == -1)
	{
		printf("error open in ft_filetostr\n");
		exit(EXIT_FAILURE);
	}
	
	fstat(fd, &st);
	if ((size = st.st_size) > MAX_FILE_SIZE)
	{
		printf("file must not exceed 200mo\n");
		exit(EXIT_FAILURE);
	}

	if (!(str = malloc(size + 1)))
	{
		printf("malloc error in ft_filetostr\n");
		exit(EXIT_FAILURE);
	}
	if ((read(fd, str, size)) < 0)
	{
		printf("error read file in ft_filetostr\n");
		exit(EXIT_FAILURE);
	}
	str[size] = '\0';
	if (close(fd) == -1)
	{
		printf("error close in ft_filetostr\n");
		exit(EXIT_FAILURE);
	}
	return (str);
}