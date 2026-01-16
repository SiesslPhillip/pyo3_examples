use pyo3::{PyResult, pyfunction};

fn divide_and_concur(input_value: i64, input_vec: Vec<i64>) -> bool {
    if input_value == input_vec[(input_vec.len() - 1) / 2] {
        return true;
    } else if input_value < input_vec[0] || input_value > input_vec[input_vec.len() - 1] {
        return false;
    } else {
        let new_search_vec: Vec<i64> = {
            if input_value < input_vec[(input_vec.len() - 1) / 2] {
                input_vec[..(input_vec.len() - 1) / 2].to_vec()
            } else {
                input_vec[(input_vec.len() - 1) / 2..].to_vec()
            }
        };
        return divide_and_concur(input_value, new_search_vec);
    }
}

#[pyfunction]
pub fn binary_search<'py>(input_value: i64, input_list: Vec<i64>) -> PyResult<bool> {
    let result = divide_and_concur(input_value, input_list);
    Ok(result)
}
