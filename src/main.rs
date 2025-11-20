use archividian as arv;


fn main()
{
    println!("Running archividian");

    let out = arv::run();

    match out {
        Err(e) => println!("{:?}", e),
        Ok(_)  => println!("DONE!")
    }
}
