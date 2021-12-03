/*
 * Runtime: 8 ms, faster than 100.00% of Rust online submissions for Number of Islands.
 * Memory Usage: 9 MB, less than 56.99% of Rust online submissions for Number of Islands.
*/
pub fn num_islands(grid: Vec<Vec<char>>) -> i32{
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited_cache = vec![false; rows * cols ];

    fn dive(grid: &Vec<Vec<char>>, i:usize, j:usize, visited_cache:&mut Vec<bool>, rows:usize, cols:usize){
        if  i >= rows || j >= cols || grid[i][j] != '1' || visited_cache[i*cols+j]{
            return;
        }

        visited_cache[i * cols + j] = true;

        dive(grid, i+1, j, visited_cache, rows, cols);
        dive(grid, i-1, j, visited_cache, rows, cols);
        dive(grid, i, j+1, visited_cache, rows, cols);
        dive(grid, i, j-1, visited_cache, rows, cols);
    }

    //For every item in the grid, 
    //if we find land, recursively search all directions for other land,
    //marking it as we go.
    
    let mut accum:i32 = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' && !visited_cache[i*cols + j] {
                accum+=1;
                dive(&grid, i, j, &mut visited_cache, rows, cols);
            }
        }
    }

    return accum;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testit(input: Vec<Vec<char>>, expected: i32){
       assert_eq!(num_islands(input), expected);
    }

    /*
    #[test]
    fn num_islands_unhappy() {
        let grid = vec![
          vec!['1','1','0','0','0'],
          vec!['1','1','0','0','0'],
          vec!['0','0','1','0','0'],
          vec!['0','0','0','1','1']
        ];

       testit(grid, 3);
    }
    */

    #[test]
    fn num_islands_unhappy() {
        let mut grid = Vec::new();
        let ogrid = [
          ['1','1','0','0','0'],
          ['1','1','0','0','0'],
          ['0','0','1','0','0'],
          ['0','0','0','1','1']
        ];

        for i in 0..ogrid.len(){
            let mut row = Vec::new();
            for j in 0..ogrid[0].len(){
                row.push(ogrid[i][j]);
            }
            grid.push(row);
        }

       assert_eq!(1,1);
       assert_eq!(num_islands(grid), 3);

    }

}
