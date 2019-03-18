use crate::types::Puzzle;

// Returns the sum of distance between the initial position and the goal position of each tiles
pub fn manhattan(initial: &Puzzle, goal: &Puzzle, size: usize) -> usize
{
    return initial.iter().enumerate().fold(0, | acc, (i, x) |
    {
        let pos_i = i as isize;
        let pos_g = goal[*x] as isize;
        let n = size as isize;
        acc + ((pos_i / n - pos_g / n).abs()
            + (pos_i % n - pos_g % n).abs()) as usize
    });
}

#[cfg(test)]
mod tests
{
    use crate::types::Puzzle;

    #[test]
    fn manhattan_0()
	{
        let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let goal: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert_eq!(super::manhattan(&initial, &goal, 3), 0);
    }

    #[test]
    fn manhattan_2()
	{
        let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let goal: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert_eq!(super::manhattan(&initial, &goal, 3), 2);
    }

    #[test]
    fn manhattan_10()
	{
        let initial: Puzzle = vec![2, 1, 8, 4, 6, 5, 7, 3, 0];
        let goal: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert_eq!(super::manhattan(&initial, &goal, 3), 10);
    }
}