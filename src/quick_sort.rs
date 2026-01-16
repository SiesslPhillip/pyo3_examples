use pyo3::{pyfunction, PyResult};
use crate::sorting;

#[pyfunction]
pub fn quick_sort_py(mut input_vec: Vec<i64>) -> PyResult<Vec<i64>> {
    let length = input_vec.len();
    let result = quick_sort(input_vec.as_mut(), 0, length);
    Ok(result.to_vec())
}


fn quick_sort(input_vec: &mut Vec<i64>, start: usize, end: usize) -> &Vec<i64> {
    let length: usize = input_vec.len();
    if length < 10 {
        let x: i64 = input_vec[start] + input_vec[end] + input_vec[(end-start)/2];
        let mut i: usize = end;
        let mut j: usize = start;
        while j >= i && i <= j {
            if input_vec[j] >= x && input_vec[i] <= x {
                let temp: i64 = input_vec[i];
                input_vec[i] = input_vec[j];
                input_vec[j] = temp;
            } else if input_vec[j] < x && input_vec[i] < x {
                j -= 1;
                i += 1;
            } else if input_vec[i] < x {
                i += 1;
            } else {
                j -= 1;
            }
        }
        quick_sort(input_vec, length, i);
        quick_sort(input_vec, j, length);
    } else{
        shell_sort(input_vec);
    }
    input_vec
}

fn insertion_sort(input_vec: &mut Vec<i64>) -> &Vec<i64> {
    if input_vec.len() <= 1 {
        return input_vec;
    }
    for i in 0..input_vec.len() - 1 {
        let next_value = input_vec[i + 1];
        let mut current_index_int = i as i64;
        while current_index_int > -1 && input_vec[current_index_int as usize] > next_value {
            input_vec[current_index_int as usize + 1] = input_vec[current_index_int as usize];
            current_index_int -= 1;
        }
        input_vec[(current_index_int + 1) as usize] = next_value;
    }
    input_vec
}

pub fn shell_sort(input_vec: &mut Vec<i64>) -> &Vec<i64> {
    let vec_length = input_vec.len();
    if vec_length <= 1 {
        return input_vec;
    }
    let mut step = (vec_length) / 2;
    while step > 0 {
        for i in step..vec_length - 1 {
            let next_value = input_vec[i];
            let mut current_index = i;
            while current_index >= step && input_vec[current_index - step] > next_value {
                input_vec[current_index] = input_vec[current_index - step];
                current_index -= step;
            }
            input_vec[current_index] = next_value;
        }
        step /= 2;
    }
    input_vec
}