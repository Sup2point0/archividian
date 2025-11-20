use archividian as arv;


fn main()
{
    println!("Running archividian");

    let out = arv::run();

    match out {
        Err(_) => println!("ERROR!"),
        Ok(_)  => println!("DONE!")
    }
}
