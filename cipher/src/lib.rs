/* #[derive(Debug, PartialEq)]
pub struct CipherError {
   pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    
    let x = original.chars().map(diro_saaf).collect();
    
    if x == ciphered{
        Ok(())
    }else{
        Err(CipherError { expected: x })
    }
}




fn diro_saaf(c: char) -> char{
    if c.is_lowercase(){

        let x = (25 - (c.to_ascii_lowercase() as u8 - 97)) + 97;
        
        x as char
    }else if  c.is_uppercase(){
        let x = (25 - (c.to_ascii_uppercase() as u8 - 65)) + 65;
        
        x as char
    }else{
        c
    }
}
 */


#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let decoded = decode(original);

    if decoded == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: decoded })
    }
}

const CHARS: u8 = 'z' as u8 - 'a' as u8;

fn decode(original: &str) -> String {
    original
        .chars()
        .map(|l| {
            if l.is_ascii_alphabetic() {
                let offset = (if l.is_ascii_uppercase() { 'A' } else { 'a' }) as u8;
                ((CHARS - (l as u8 - offset)) + offset) as char
            } else {
                l
            }
        })
        .collect()
}