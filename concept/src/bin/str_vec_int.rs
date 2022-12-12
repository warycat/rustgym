fn to_int(s:&str)->Option<i32>{
    // s.parse().unwrap() // this will panic if s is not a number
    // s.parse().expect("Not a number") // this will panic with the message "Not a number"
    // s.parse().unwrap_or(0) // this will return 0 if s is not a number
    s.parse().ok() // this will return None if s is not a number
}

fn sum_str_vec(strs:Vec<String>)->String{
    let mut accum =0i32;
    for s in strs{
        // accum += to_int(&s);
        // accum += match to_int(&s){
        //     Some(n)=>n,
        //     None=>0
        // }
        // if let Some(val) =  to_int(&s){
        //     accum += val;
        // }
        accum += to_int(&s).unwrap_or(0); // this will return 0 if None is returned else the Some value is returned and added to accum
    }

    return accum.to_string();
}

fn main(){
    let strs= vec![String::from("1"),String::from("abc"),String::from("3")];
    let total = sum_str_vec(strs);
    println!("{:?}",total);
}