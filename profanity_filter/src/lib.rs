pub fn check_ms(message: &str) -> Result<&str, &str> {

  if message.contains("stupid"){
    Ok(message)
  }else{
    Err("ERROR: illegal")
  }
}