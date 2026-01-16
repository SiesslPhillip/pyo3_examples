mod binarytree;
mod linkedlist;
mod morsecode;
mod search;
mod sorting;
mod quick_sort;

use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pyfunction]
fn sort_string(input: String) -> PyResult<String> {
    let mut words: Vec<&str> = input.split_ascii_whitespace().collect();
    let number_of_words = words.len();
    for i in 0..number_of_words {
        let mut min_idx = i;
        for y in (i + 1)..number_of_words {
            if words[y] < words[min_idx] {
                min_idx = y;
            }
        }
        words.swap(i, min_idx);
    }
    let result = String::from(words.join(" "));
    Ok(result)
}

// mirror this string using recursion
#[pyfunction]
fn mirror_string(input_string: String) -> PyResult<String> {
    let closure_string = |input_string: String| {
        let reduced_string = &input_string[..input_string.len() - 1];
        return reduced_string.to_owned();
    };
    if input_string.len() <= 1 {
        Ok(input_string)
    } else {
        let result_literal = &input_string[input_string.len() - 1..];
        let mut result_string: String = Default::default();
        result_string.push_str(result_literal);
        result_string.push_str(mirror_string(closure_string(input_string))?.as_str());
        Ok(result_string)
    }
}

#[pymodule]
fn python_rust(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<linkedlist::LinkedList>()?;
    m.add_class::<binarytree::BinaryTree>()?;
    m.add_class::<morsecode::BinaryMorseTree>()?;
    m.add_function(wrap_pyfunction!(sort_string, m)?)?;
    m.add_function(wrap_pyfunction!(search::binary_search, m)?)?;
    m.add_function(wrap_pyfunction!(mirror_string, m)?)?;
    m.add_function(wrap_pyfunction!(sorting::selection_sort, m)?)?;
    m.add_function(wrap_pyfunction!(sorting::insertion_sort, m)?)?;
    m.add_function(wrap_pyfunction!(sorting::shell_sort, m)?)?;
    m.add_function(wrap_pyfunction!(quick_sort::quick_sort_py, m)?)?;

    Ok(())
}
