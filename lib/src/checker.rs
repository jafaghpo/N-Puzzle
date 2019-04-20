use crate::types::Map;

// Returns the sum of inversions for each tiles except the empty one
fn get_inversions(puzzle: &Map) -> usize
{
    let mut inversions = 0;
    for i in 0..puzzle.len() - 1
	{
        for j in i + 1..puzzle.len()
		{
            if puzzle[i] == 0 || puzzle[j] == 0 { continue }
            if puzzle[i] > puzzle[j] { inversions += 1 }
        }
    }
    return inversions;
}

// The solvability of a puzzle is explaned here (including inversions):
// http://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html
pub fn is_solvable(initial: &Map, goal: &Map, size: usize) -> bool
{
    let mut initial_inv = get_inversions(initial);
    let mut goal_inv = get_inversions(goal);

	// If the size is even, we take into account the position of the empty tile
    if size % 2 == 0
	{
        initial_inv += initial.iter().position(|x| *x == 0).unwrap() / size;
        goal_inv += goal.iter().position(|x| *x == 0).unwrap() / size;
    }
	// The "total" polarity (depending on the polarity of the size)
	// of a solvable puzzle MUST be the same as that of its final state
    return initial_inv % 2 == goal_inv % 2;
}


#[cfg(test)]
mod tests
{
    use crate::types::Map;

    #[test]
    fn inversions()
	{
        let a: Map = vec![4, 3, 2, 1];

        assert_eq!(super::get_inversions(&a), 6);
    }

    #[test]
    fn inversions_ignore_zero()
	{
        let a: Map = vec![3, 2, 1, 0];

        assert_eq!(super::get_inversions(&a), 3);
    }

    #[test]
    fn inversions_none()
	{
        let a: Map = vec![1, 2, 3, 4];

        assert_eq!(super::get_inversions(&a), 0);
    }

    #[test]
    fn is_solvable_inverted()
	{
        let a: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Map = vec![1, 3, 2, 4, 5, 6, 7, 8, 0];

        assert!(!super::is_solvable(&a, &b, 3));
        assert!(!super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_identity()
	{
        let a: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4()
	{
        let a: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4_identity()
	{
        let a: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }
}