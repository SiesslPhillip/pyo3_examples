use pyo3::{pyfunction, PyResult};

#[pyfunction]
pub fn quick_sort_py(mut input_vec: Vec<i64>) -> PyResult<Vec<i64>> {
    let length = input_vec.len();
    if length > 1 {
        quick_sort(&mut input_vec, 0, length - 1);
    }
    Ok(input_vec)
}

fn quick_sort(input_vec: &mut Vec<i64>, start: usize, end: usize) -> &Vec<i64> {
    if start >= end {
        return input_vec;
    }

    let length: usize = end - start + 1;

    if length <= 100 {
        shell_sort(input_vec, start, end);
        return input_vec;
    }

    let mid = start + (end - start) / 2;
    let x: i64 = input_vec[mid];
    input_vec.swap(mid, end);

    let mut i: usize = start;
    let mut j: usize = start;

    while j < end {
        if input_vec[j] <= x {
            input_vec.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    input_vec.swap(i, end);

    if i > start {
        quick_sort(input_vec, start, i - 1);
    }
    if i + 1 < end {
        quick_sort(input_vec, i + 1, end);
    }

    input_vec
}

fn shell_sort(input_vec: &mut Vec<i64>, start: usize, end: usize) {
    let vec_length = end - start + 1;
    if vec_length <= 1 {
        return;
    }

    let mut step = vec_length / 2;
    while step > 0 {
        let mut i = start + step;
        while i <= end {
            let next_value = input_vec[i];
            let mut current_index = i;

            while current_index >= start + step && input_vec[current_index - step] > next_value {
                input_vec[current_index] = input_vec[current_index - step];
                current_index -= step;
            }

            input_vec[current_index] = next_value;
            i += 1;
        }
        step /= 2;
    }
}
