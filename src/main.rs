use std::{
    env,
};

use archividian;


fn main()
{
    println!("Running archividian");

    let args: Vec<String> = env::args().collect();
    let out = archividian::run(args);

    match out {
        Err(_) => println!("ERROR!"),
        Ok(_)  => println!("DONE!")
    }
}
