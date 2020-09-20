use std::cmp;

fn maximum_path_sum(triangle: &mut [&mut [usize]]) -> usize {
    let length = triangle.len();
    if length == 1 {
        triangle[0][0]
    } else {
        let (last_row, head) = triangle.split_last_mut().unwrap();
        let (before_last, _) = head.split_last_mut().unwrap();
        before_last
            .iter_mut()
            .enumerate()
            .for_each(|(n, i)| *i += cmp::max(last_row[n], *last_row.get(n + 1).unwrap_or(&0)));
        maximum_path_sum(&mut triangle[..(length - 1)])
    }
}

fn main() {
    maximum_path_sum(&mut [&mut [1]]);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(
            23,
            maximum_path_sum(&mut [&mut [3], &mut [7, 4], &mut [2, 4, 6], &mut [8, 5, 9, 3]])
        );
        assert_eq!(
            1074,
            maximum_path_sum(&mut [
                &mut [75],
                &mut [95, 64],
                &mut [17, 47, 82],
                &mut [18, 35, 87, 10],
                &mut [20, 04, 82, 47, 65],
                &mut [19, 01, 23, 75, 03, 34],
                &mut [88, 02, 77, 73, 07, 63, 67],
                &mut [99, 65, 04, 28, 06, 16, 70, 92],
                &mut [41, 41, 26, 56, 83, 40, 80, 70, 33],
                &mut [41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                &mut [53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
                &mut [70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
                &mut [91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
                &mut [63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
                &mut [04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]
            ])
        );
    }
}
