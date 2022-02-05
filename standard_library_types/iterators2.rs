// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// I AM DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();

    let first_cap = match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };

    match first_cap {
        i if i.chars().count() == 0 => "".to_string(), 
        _ => format!("{}{}", first_cap, &input[1 ..])
    }

}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {  

    let mut my_vec = Vec::new();

    for w in words.iter() {
        let mut c = w.chars();

        let cap = match c.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string(),
        };

        my_vec.push(format!("{}{}", cap, &w[1 ..]));
    }
    
    my_vec

}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {

    let mut new_string = String::new();

    for w in words.iter() {
        let mut c = w.chars();

        let cap = match c.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string(),
        };        

        new_string = format!("{}{}{}",new_string, cap, &w[1 ..]);        

    }

    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
