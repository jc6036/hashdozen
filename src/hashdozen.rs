// TODO: Rustdoc comments, work out how to structure package for crates.io
// TODO: Add tests
// TODO: Add support for HashMap
// TODO: impl Hasher for all rust primitives

// Primary Hasher Implementation
fn run_hash (mut input: (Vec<u8>, Vec<u8>)) -> String {
    // Pad out to multiple of 6 bytes
    pad (&mut input);

    // Compress each vec into one block
    let datablocks = compress(input);

    // Finally, combine the main data block and the salt
    let finaldata = combine(datablocks);

    // Now output as a string
    return format_hash(finaldata);
}

fn pad(data: &mut (Vec<u8>, Vec<u8>)) {
    data.0.extend(data.0.len().to_be_bytes());
    while data.0.len() % 6 > 0 {
        data.0.push(255);
    }

    data.1.extend(data.1.len().to_be_bytes());
    while data.1.len() % 6 > 0 {
        data.1.push(255);
    }
}

fn compress(mut data: (Vec<u8>, Vec<u8>)) -> (Vec<u8>, Vec<u8>) {
    let mut mainblock: Vec<u8> = data.0[data.0.len() - 6..].to_vec();
    let mut saltblock: Vec<u8> = data.1[data.1.len() - 6..].to_vec();

    data.0.truncate(data.0.len() - 6);

    data.1.truncate(data.1.len() - 6);
    while data.0.len() >= 6 {
        // XOR the first 6 bytes of data into our datablock
        mainblock = mainblock
                        .iter()
                        .zip(data.0[data.0.len() - 6..].iter())
                        .map(|(&x1, &x2)| ((x1.rotate_right(x2.into())) ^ (x2.rotate_right(x1.into()))))
                        .collect();
        data.0.truncate(data.0.len() - 6);
    }
    while data.1.len() >= 6 {
        saltblock = saltblock
                        .iter()
                        .zip(data.1[data.1.len() - 6..].iter())
                        .map(|(&x1, &x2)| ((x1.rotate_right(x2.into())) ^ (x2.rotate_right(x1.into()))))
                        .collect();
        data.1.truncate(data.1.len() - 6);
    }

    return (mainblock, saltblock);
}

fn combine (data: (Vec<u8>, Vec<u8>)) -> Vec<u8> {
    data.0
        .iter()
        .zip(data.1.iter())
        .map(|(&x1, &x2)| ((x1.rotate_right(x2.into())) ^ (x2.rotate_right(x1.into()))))
        .collect()
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

// Type Implementations

// std::String
impl Hasher for String {
    fn convert_to_bytes (&self) -> Vec<u8> {
        let mut newvec: Vec<u8> = Vec::new();
        for i in self.as_bytes() {
            newvec.push(*i);
        }
        return newvec;
    }
}

impl Hasher for usize {
    fn convert_to_bytes(&self) -> Vec<u8> {
        let mut newvec: Vec<u8> = Vec::new();
        newvec.extend(self.to_be_bytes());

        return newvec;
    }
}

// API
// This trait must be implemented for any type you wish to be compatible with the hasher
/// This needs implemented in order to run hash on your custom types. 
/// Already implemented for rust primitives and String.
/// 
/// ```
/// // Example of how you might implement this to work on a simple struct
/// ```
pub trait Hasher {
    fn convert_to_bytes (&self) -> Vec<u8>;
}

/// This is the primary public interface for HashDozen.
/// 
/// The function declaration should be fairly self explanatory.
/// You may notice that the types are trait bounded to Hasher. If you have custom types you wish to
/// implement, please see the documentation for the trait.
/// 
/// It expects an input and a salt, every time. It's up to you how to generate the salt, and they can 
/// be different types.
/// 
/// ```
/// // Usage Example
/// let test1 = 23890423984;
/// let test2 = String::from("Test2asfasfdsaf");
/// println!("{}",hashdozen::hash(&test1, &test2).unwrap()); 
/// ```
pub fn hash<'a, D: Hasher, S: Hasher> (input: &D, salt: &S) -> Result<String, &'a str> {
    let hashinput = input.convert_to_bytes();
    let saltinput = salt.convert_to_bytes();

    if hashinput.len() == 0 || saltinput.len() == 0 {
        let ret: &'a str = "One or more inputs were empty.";
        return Result::Err(ret);
    }

    return Ok(run_hash((hashinput, saltinput)));
}