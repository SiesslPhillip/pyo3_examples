use pyo3::{PyResult, pyfunction};

#[pyfunction]
pub fn selection_sort(mut input_vec: Vec<i64>) -> PyResult<Vec<i64>> {
    if input_vec.len() <= 1 {
        return Ok(input_vec);
    }
    for i in 0..input_vec.len() - 1 {
        let mut to_swap: usize = i;
        for y in i..input_vec.len() {
            if input_vec[to_swap] > input_vec[y] {
                to_swap = y
            }
        }
        let swap_value = input_vec[i];
        input_vec[i] = input_vec[to_swap];
        input_vec[to_swap] = swap_value;
    }
    Ok(input_vec)
}

#[pyfunction]
pub fn shell_sort(mut input_vec: Vec<i64>) -> PyResult<Vec<i64>> {
    let vec_length = input_vec.len();
    if vec_length <= 1 {
        return Ok(input_vec);
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
    Ok(input_vec)
}

#[pyfunction]
pub fn insertion_sort(mut input_vec: Vec<i64>) -> PyResult<Vec<i64>> {
    if input_vec.len() <= 1 {
        return Ok(input_vec);
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
    Ok(input_vec)
}
