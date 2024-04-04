// Primary Hasher Implementation
fn run_hash (mut input: Vec<u8>) -> String {
    // Pad out to multiple of 6 bytes
    pad (&mut input);

    // Compress each vec into one block
    let datablocks = compress(input);

    // Now output as a string
    return format_hash(datablocks);
}

fn pad(data: &mut Vec<u8>) {
    data.extend(data.len().to_be_bytes());
    while data.len() % 6 > 0 {
        data.push(255);
    }
}

fn compress(mut data: Vec<u8>) -> Vec<u8> {
    let mut mainblock: Vec<u8> = data[data.len() - 6..].to_vec();

    data.truncate(data.len() - 6);

    while data.len() >= 6 {
        mainblock = mainblock
                        .iter()
                        .zip(data[data.len() - 6..].iter())
                        .map(|(&x1, &x2)| ((x1.rotate_right(x2.into())) ^ (x2.rotate_right(x1.into()))))
                        .collect();
        data.truncate(data.len() - 6);
    }
    return mainblock;
}


fn format_hash(data: Vec<u8>) -> String {
    let mut outstr = String::new();

    for i in data.iter().enumerate() {
        match i.0 + 1 == 2 || i.0 + 1 == 4 {
            true => outstr = format!("{outstr}{:02X}-",i.1),
            false => outstr = format!("{outstr}{:02X}",i.1),
        }
    }

    return outstr;
}


/// This is the primary public interface for HashDozen.
/// 
/// ```
/// // Usage example
/// use std::string::String;
/// use hashdozen::hashdozen;
/// 
/// let data_str = String::from("Test hash string!");
/// println!("{}", hashdozen::hash(Vec::<u8>::from(data_str)));
/// ```
/// 
/// Please note that passing in an unsalted piece of data is not recommended. You can either supply your own salt,
/// or use generate_salt() from this library. There is also a shortcut method salt_then_hash() to run both the salt
/// then this hash method in sequence.
pub fn hash (input: Vec<u8>) -> String {
    let hashinput = Vec::<u8>::from(input);  

    return run_hash(hashinput);
}

/// Takes a data value, then returns it salted.
/// 
/// Recommended to run this when creating input for the hasher, but feel free to replace it with whatever
/// salting process you wish.
/// 
/// ```
/// // An example of what your data may look like pre and post salting
/// use std::string::String;
/// use hashdozen::hashdozen;
/// 
/// let unsalted_str = String::from("Unsalted Data!");
/// let unsalted_data = Vec::<u8>::from(unsalted_str);
/// println!("{:x?}", unsalted_data); // Let's see what our unsalted String looks like inside the vector...
/// let salted_data = hashdozen::generate_salted_input(unsalted_data); // Salt it
/// println!("{:x?}", salted_data);
/// 
/// // Note that this returns a Vec<u8>, because not all salts are guranteed to be valid UTF representations.
/// // You can try to convert if you want to see it as char data, but you will need to do error handling on the transform.
/// let salted_str = String::from_utf8(salted_data).unwrap();
/// println!("{}", salted_str);
/// ```
pub fn generate_salted_input (input: Vec<u8>) -> Vec<u8> {
    let mut input = Vec::<u8>::from(input);//input.convert_to_bytes();

    let salt: Vec<u8> = input.iter().zip(input.iter().rev()).map(|(x1, x2)| (x1 | x2) ^ (x1 & x2)).collect();

    input.append(&mut salt.clone());

    input
}

/// A shortcut to add a salt to your input then hash it rather than doing so separately.
/// 
/// ```
/// // Usage
/// use std::string::String;
/// use hashdozen::hashdozen;
/// 
/// let data_str = String::from("Hash Foo Hash Bar");
/// println!("{}", hashdozen::salt_then_hash(Vec::<u8>::from(data_str)));
/// ```
pub fn salt_then_hash(input: Vec<u8>) -> String {
    let saltydata = generate_salted_input(input);

    hash(saltydata)
}