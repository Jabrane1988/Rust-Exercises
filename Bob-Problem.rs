pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let question = message.ends_with("?");
    let yell = message.contains(char::is_alphabetic) && message == message.to_uppercase();

    if message.is_empty() {"Fine. Be that way!"}
    else if question && yell {"Calm down, I know what I'm doing!"}
    else if question {"Sure."}
    else if yell {"Whoa, chill out!"}
    else {"Whatever."}
    
}
