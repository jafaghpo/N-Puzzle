/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   file.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/02/19 13:01:14 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/04 17:26:01 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "npuzzle.h"

char		*ft_filetostr(char *file)
{
	int			fd;
	char		*str;
	struct stat st;
	size_t		size;

	str = NULL;
	if ((fd = open(file, O_RDONLY)) == -1)
		exit_program("error open in ft_filetostr\n");
	fstat(fd, &st);
	if ((size = st.st_size) > MAX_FILE_SIZE)
		exit_program("file must not exceed 200mo\n");
	if (!(str = malloc(size + 1)))
		exit_program("malloc error in ft_filetostr\n");
	if ((read(fd, str, size)) < 0)
		exit_program("error read file in ft_filetostr\n");
	str[size] = '\0';
	if (close(fd) == -1)
		exit_program("error close in ft_filetostr\n");
	return (str);
}
