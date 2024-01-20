use serde::de::DeserializeOwned;

use crate::{
    helpers::terminal::PrintCommand, models::general::llm::Message,
    services::call_request::call_gpt,
};

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

/// Call the AI task request
pub async fn ai_task_request(
    message_context: &str,
    agent_role: &str,
    agent_statement: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
    temperature: f32,
) -> Result<String, anyhow::Error> {
    let extended_msg = extend_ai_function(function_pass, message_context);
    PrintCommand::AICall.print_agent_message(agent_role, agent_statement)?;
    match call_gpt(vec![extended_msg.clone()], temperature).await {
        Ok(resp) => Ok(resp),
        Err(err) => {
            println!("AI call failed ({err}) Trying again...");
            call_gpt(vec![extended_msg], temperature).await
        }
    }
}

/// Deserialize the AI task request
pub async fn ai_task_request_deserialize<T: DeserializeOwned>(
    message_context: &str,
    agent_role: &str,
    agent_statement: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
    temperature: f32,
) -> Result<T, anyhow::Error> {
    let resp = ai_task_request(
        message_context,
        agent_role,
        agent_statement,
        function_pass,
        temperature,
    )
    .await?;
    Ok(serde_json::from_str(&resp)?)
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

    #[tokio::test]
    async fn test_ai_task_request() -> Result<(), anyhow::Error> {
        let message_context = "I need a website that lets users login and logout. It needs to look fancy and accept payments.";
        let agent_role = "Managing Agent";
        let agent_statement = "Hello, I am a Managing Agent.";
        let function_pass = print_user_input_to_goal;
        let temperature = 0.5;
        let resp: String = ai_task_request(
            message_context,
            agent_role,
            agent_statement,
            function_pass,
            temperature,
        )
        .await?;
        dbg!(&resp);
        assert!(resp.len() > 20);
        Ok(())
    }
}
