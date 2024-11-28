use std::io::{self};
fn main() {
    let mut data: Vec<Vec<String>> = Vec::new();

    loop {
        let input_u=input();

        match input_u.as_str() {
              "ls" => {
                println!("your data is");
                for items in &data  {
                    println!("type= {} email= {} password= {}",items[0],items[1],items[2]);
                }
                println!("");
              },

              "add" =>{
                println!("");
                      println!("please enter type ");
                      let input_type=input();
                      println!("please enter email");
                      let input_email=input();
                      println!("please enter password");
                      let input_password=input();

                      data.push(vec![input_type,input_email,input_password]);
                     
                     println!("your data is Add");

              },
              "update"=>{
                println!("enter update type");
                let search_type=input();
                let mut fuond=false;
                println!("");
                  for items in &mut data  {
                      if  items[0] ==  search_type {
                        fuond=true;

                        println!("your type is found");
                        println!("your last data is");
                        println!("type= {}",items[0]);
                        println!("email= {}",items[1]);
                        println!("password= {}",items[2]);

                          println!("");
                           println!("please enter new type");
 
                          let new_type=input();
                          if !new_type.is_empty() {
                              items[0]=new_type;
                          }
                          println!("");
                          println!("please enter new eamil");
                          let new_email=input();
                          if !new_email.is_empty() {
                              items[0]=new_email;
                              println!("");
                              println!("please enter new password");
                          }let new_password=input();
                          if !new_password.is_empty() {
                              items[0]=new_password;
                          }

                          println!("your data is update");

                      }
                     
                          
;                     
                  }

                  if !fuond {
                      println!("sorry not found this |{}| type please enter update to try",search_type);
                  }



              },
              "delete" =>{
                println!("enter delete type");
                let search_type=input();
                println!("");
                
    data.retain(|items|{
        if items[0] == search_type {
         println!("");
         println!("data found");
         println!("please enter 'ok' to delete data");
         let confirm=input();
         if confirm == "ok" {
            println!("delete sucsesfull");
            false
         }else {
            println!("detete canclled");
            true
            
        }
    }else {
        true
    }
});
                
                


              },
              "clear" =>{
                clear_screen();
    println!("Screen cleared!");
              }
              
              "exit" => break,


            _ => println!("Please type somthing")
        }
    }


}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please enter the text");
    input.trim().to_string()
}


fn clear_screen() {
    // Use OS-specific commands to clear the screen
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
