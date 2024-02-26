/* 滑动窗口 */
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut col: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for i in 0..9 {
            for j in 0..9 {
                let t = board[i][j];
                let box_index = j / 3 + (i / 3) * 3;

                if t == '.' {
                    continue;
                }
                if row[i].contains(&t) {
                    return false;
                }
                if col[j].contains(&t) {
                    return false;
                }
                if boxes[box_index].contains(&t) {
                    return false;
                }

                row[i].insert(t);
                col[j].insert(t);
                boxes[box_index].insert(t);
            }
        }

        return true;
    }

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        if matrix.len() == 0 {
            return res;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let iter_num = std::cmp::min(rows, cols) / 2; //迭代次数  如果是奇数的话 最后一次就是一个行或列 单独考虑

        for iter in 0..iter_num {
            //上面遍历
            for col_id in iter..(cols - iter - 1) {
                res.push(matrix[iter][col_id]);
            }
            //右边遍历
            for row_id in iter..(rows - iter - 1) {
                res.push(matrix[row_id][cols - iter - 1]);
            }
            //下方遍历
            for col_id in (iter + 1..cols - iter).rev() {
                res.push(matrix[rows - iter - 1][col_id]);
            }
            //左边遍历
            for row_id in (iter + 1..rows - iter).rev() {
                res.push(matrix[row_id][iter]);
            }
        }

        //考虑剩下单行的情况
        if rows <= cols && (rows & 1 > 0) {
            for col_id in iter_num..cols - iter_num {
                res.push(matrix[iter_num][col_id]);
            }
        }

        //考虑剩下单列的情况
        if cols < rows && (cols & 1 > 0) {
            for row_id in iter_num..rows - iter_num {
                res.push(matrix[row_id][iter_num]);
            }
        }

        res
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n / 2) {
            for j in 0..((n + 1) / 2) {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = temp;
            }
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row: HashSet<usize> = HashSet::new();
        let mut cow: HashSet<usize> = HashSet::new();

        for (k, v) in matrix.iter().enumerate() {
            for (k2, v2) in v.iter().enumerate() {
                if *v2 == 0 {
                    row.insert(k);
                    cow.insert(k2);
                }
            }
        }

        for j in row {
            for i in 0..matrix[0].len() as usize {
                matrix[j][i] = 0;
            }
        }
        for j in cow {
            for i in 0..matrix.len() {
                matrix[i][j] = 0;
            }
        }

        // matrix
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let lx = board[0].len();
        let ly = board.len();
        for i in 0..ly {
            for j in 0..lx {
                for y in [-1, 0, 1] {
                    for x in [-1, 0, 1] {
                        // X Y Block
                        if !(i as i32 + x < 0 || i as i32 + x >= ly as i32 || // h
                        j as i32 + y < 0 || j as i32 + y >= lx as i32// v 
                        || (x == 0 && y == 0))
                        // mid
                        {
                            board[i][j] +=
                                board[(i as i32 + x) as usize][(j as i32 + y) as usize] << 1 & 0b10
                        }
                    }
                } // X Y block end
            }
        }

        for i in 0..ly {
            for j in 0..lx {
                board[i][j] = match board[i][j] >> 1 {
                    0 => 0,
                    1 => 0,
                    2 => {
                        if (board[i][j] & 1) != 0 {
                            1
                        } else {
                            0
                        }
                    }
                    3 => 1,
                    4 => 0,
                    5 => 0,
                    6 => 0,
                    7 => 0,
                    8 => 0,
                    _ => 0,
                };
            }
        }
    }
}

fn main() {
    /* 34. is_valid_sudoku */
    let board = vec![
        ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ];
    let converted_board: Vec<Vec<char>> = board
        .iter()
        .map(|row| row.iter().map(|&c| c.chars().next().unwrap()).collect())
        .collect();

    let ret = Solution::is_valid_sudoku(converted_board);
    println!("{}", ret);

    /* 35. spiral_order */
    let matrix: Vec<[i32; 3]> = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let converted_matrix: Vec<Vec<i32>> = matrix.iter().map(|array| array.to_vec()).collect();

    let ret = Solution::spiral_order(converted_matrix);
    println!("{:?}", ret);

    /* 36. rotate */
    let matrix: Vec<[i32; 3]> = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut converted_matrix: Vec<Vec<i32>> = matrix.iter().map(|array| array.to_vec()).collect();
    Solution::rotate(&mut converted_matrix);
    println!("{:?}", converted_matrix);

    /* 37. set_zeroes */
    let matrix: Vec<[i32; 3]> = vec![[1, 2, 3], [4, 0, 6], [7, 8, 9]];
    let mut converted_matrix: Vec<Vec<i32>> = matrix.iter().map(|array| array.to_vec()).collect();
    Solution::set_zeroes(&mut converted_matrix);
    println!("{:?}", converted_matrix);

    /* 38. game_of_life */
    let matrix = vec![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
    let mut converted_matrix: Vec<Vec<i32>> = matrix.iter().map(|array| array.to_vec()).collect();
    Solution::game_of_life(&mut converted_matrix);
    println!("{:?}", converted_matrix);
}
