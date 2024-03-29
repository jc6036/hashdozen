use hashdozen::hashdozen;

fn main () {
    let test1 = String::from("Different stuff. This is a super massive string just to test some of my hashing nonsense.");
    let test2 = String::from("Test2asfasfdsaf");
    println!("{}",hashdozen::hash(&test1, &test2));
}