fn foi_editada(str1: &str, str2: &str) -> bool {
    let len1 = str1.len(); 
    let len2 = str2.len();

    
    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    }

    let mut dif_encontrada = false; 
    let mut i = 0; 
    let mut j = 0;

    let chars1: Vec<char> = str1.chars().collect(); 
    let chars2: Vec<char> = str2.chars().collect();

    while i < len1 && j < len2 {
        if chars1[i] != chars2[j] {
            if dif_encontrada {
                return false; 
            }
            dif_encontrada = true;

           
            if len1 == len2 {
                i += 1;
                j += 1;
            } 
            
            else if len1 > len2 {
                i += 1;
            } 
            
            else {
                j += 1;
            }
        } else {
            i += 1;
            j += 1;
        }
    }
    true
}

fn main(){
    let str1 = "pale";
    let str2 = "ple";
    println!("Strings estão a uma edição de distância: {}", foi_editada(str1, str2));
 
    let str3 = "pales";
    let str4 = "pale";
    println!("Strings estão a uma edição de distância: {}", foi_editada(str3, str4));
 
    let str5 = "pale";
    let str6 = "bale";
    println!("Strings estão a uma edição de distância: {}", foi_editada(str5, str6));
 
    let str7 = "pale";
    let str8 = "bibo";
    println!("Strings estão a uma edição de distância: {}", foi_editada(str7, str8));
}