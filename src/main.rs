static ALPHABET : [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

fn vigenere_encrypt(plaintext : String, key : String) -> String {
    let mut ciphertext = String::new();

    let key_len = key.chars().count();
    let mut key_iter = 0;
    
    for c in plaintext.chars() {

        let m = ALPHABET.iter().position(|&x| x == c).unwrap();
        let k = ALPHABET.iter().position(|&x| x == key.chars().nth(key_iter).unwrap()).unwrap();
        let letter = (m + k) % 26; 
        ciphertext.push(ALPHABET[letter]);

        if key_iter == key_len - 1 {
            key_iter = 0;
        } else {
            key_iter+=1;
        }
    }

    ciphertext
}


fn vigenere_decrypt(ciphertext : String, key : String) -> String {
    let mut plaintext = String::new();

    let key_len = key.chars().count();
    let mut key_iter = 0;
    
    for c in ciphertext.chars() {

        let m = ALPHABET.iter().position(|&x| x == c).unwrap();
        let k = ALPHABET.iter().position(|&x| x == key.chars().nth(key_iter).unwrap()).unwrap();
        let mut temp = (m as i32) - (k as i32);
        if temp < 0 {
            temp = 26-(temp*-1);
        }
        println!("{}", temp);
        
        let letter = temp % 26; 
        plaintext.push(ALPHABET[(letter as usize)]);

        if key_iter == key_len - 1 {
            key_iter = 0;
        } else {
            key_iter+=1;
        }
    }

    plaintext 
}


fn main() {
    println!("Ciphertext: {}" , vigenere_encrypt(String::from("TESTPLAINTEXT"), String::from("KEY")));
    println!("Plaintext: {}" , vigenere_decrypt(vigenere_encrypt(String::from("TESTPLAINTEXT"), String::from("KEY")), String::from("KEY")));
}
