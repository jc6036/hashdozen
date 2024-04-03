// TODO: Rustdoc comments, work out how to structure package for crates.io
// TODO: Add tests
// TODO: Add support for HashMap
// TODO: impl Hasher for all rust primitives
// TODO: Remove salt, add utility method to generate a salted value as an option
// TODO: Alter the pad to cut down on repeated pad values

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

impl Hasher for Vec<u8> {
    fn convert_to_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}


/// This needs implemented in order to run hash on your custom types. 
/// Already implemented for rust primitives and String.
pub trait Hasher {
    fn convert_to_bytes (&self) -> Vec<u8>;
}

/// This is the primary public interface for HashDozen.
/// 
/// The function declaration should be fairly self explanatory.
/// You may notice that the type is trait bounded to Hasher. If you have custom types you wish to
/// implement, please see the documentation for the trait.
pub fn hash<'a, T: Hasher> (input: &T) -> Result<String, &'a str> {
    let hashinput = input.convert_to_bytes();    

    if hashinput.len() == 0 {
        let ret: &'a str = "Input can not be empty.";
        return Result::Err(ret);
    }

    return Ok(run_hash(hashinput));
}

/// Takes a data value that must implement Hasher and convert_to_bytes, then returns it salted.
/// 
/// Recommended to run this when creating input for the hasher, but feel free to replace it with whatever
/// salting process you wish.
pub fn generate_salt<T: Hasher> (input: &T) -> Vec<u8> {
    let mut input = input.convert_to_bytes();

    let salt: Vec<u8> = input.iter().zip(input.iter().rev()).map(|(x1, x2)| (x1 | x2) ^ (x1 & x2)).collect();

    input.append(&mut salt.clone());

    input
}