use crate::models::general::llm::Message;

/// Extend the AI function to encourage specific output.
pub fn extend_ai_function(function: fn(&str) -> &'static str, function_input: &str) -> Message {
    // "build a website that handles users logging in and logging out and accepts payments"
    let ai_function_str = function(function_input);

    // extend the string to encourage only printing the output
    #[rustfmt::skip]
    let msg = format!(
    "FUNCTION {ai_function_str} 
    INSTRUCTION: You are a function printer.  
    You ONLY print the results of functions. Nothing else. No commentary.  
    Here is the input to the function: {function_input}. 
    Print out what the function will return."
    );
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::manager::print_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let output = extend_ai_function(print_user_input_to_goal, "I need a website that lets users login and logout. It needs to look fancy and accept payments.");

        assert_eq!(output.role, "system".to_string());
    }
}
