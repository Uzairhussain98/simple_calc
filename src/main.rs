use std::io::stdin;

fn main(){

    loop {
    
    let mut pass = String::new();
    println!("Enter 1 To Continue Or 0 To Exit" );
    stdin().read_line(&mut pass).expect("unexpected input");
    let pass : i32 = pass.trim().parse().unwrap();
    
    if pass == 1 {
    
    
        let mut opp = String::new();
        println!("Enter + for add ,\nEnter - for sub , \nEnter / for div , \nEnter * for multiply or \nEnter ^ for exponent " );
        stdin().read_line(&mut opp).expect("unexpected input");
        let opp : char = opp.trim().parse().unwrap();
    
    
    
        let mut value1 = String::new();
        println!("Enter first value" );
        stdin().read_line(&mut value1).expect("unexpected input");
        let value1 : u32 = value1.trim().parse().unwrap();
    
    
        let mut value2 = String::new();
        println!("Enter second value" );
        stdin().read_line(&mut value2).expect("unexpected input");
        let value2 : u32  = value2.trim().parse().unwrap();
    
    if opp == '+' {
    
    let result = value1 + value2;
    println!("{} + {} = {:#?}", value1 , value2 , result );
    
    
    }
    
    
    else if opp == '-' {
    
        let result = value1 - value2 ;
        println!("{} - {} = {:#?}", value1 , value2 , result );
        
        }
    
    
    
        else if opp == '/' {
    
            let result = value1 / value2;
            println!("{} / {} = {:#?}", value1 , value2 , result );
    
            
            }
    
            else if opp == '*' {
    
               let result = value1 * value2;
               println!("{} * {} = {:#?}", value1 , value2 , result );
    
                
                }
    
                else if opp == '^' {
    
                   let result = value1.pow(value2) ;
                   println!("{} ^ {} = {:#?}", value1 , value2 , result );                    
                    }
            
    else {
        println!("incorrent operation", );
    }
    }
     else {
         println!("bye" );
         break;
     } 
        }  
    
    }
    