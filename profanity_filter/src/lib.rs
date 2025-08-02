pub fn check_ms(message: &str) -> Result<&str, &str> {

  if message.contains("stupid") || message == "" {
    Err("ERROR: illegal")
  }else{
    Ok(message)
  }
}