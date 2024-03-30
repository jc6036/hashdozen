// Also: Tests, documentation comments, work out how to structure package for crates.io
// TODO: Choose a better pad than 255. Predictable pads are one of the number one sources of collision.
  // TODO: Second collision test after pad is updated. Current collision rate when hashing moby dick: .002% on 31k unique 'words'.
  //       This extrapolates out to 83% at 13,000,000 ((2^n/2) where n=48) keys, which is double what we want. .001% at 31k would be much better.
// TODO: Optimize
// TODO: Add tests
// TODO: Add benchmarking

// Primary Hasher Implementation
fn run_hash (mut input: (Vec<u8>, Vec<u8>)) -> String {
    // Pad out to multiple of 6 bytes
    pad (&mut input, &(255, 255));

    // Compress each vec into one block
    let datablocks = compress(input);

    // Finally, combine the main data block and the salt
    let finaldata = combine(datablocks);

    // Now output as a string
    return format_hash(finaldata);
}

fn pad(data: &mut (Vec<u8>, Vec<u8>), padvals: &(u8, u8)) {
    while data.0.len() % 6 > 0 {
        data.0.push(padvals.0);
    }

    while data.1.len() % 6 > 0 {
        data.1.push(padvals.1);
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

// API
// This trait must be implemented for any type you wish to be compatible with the hasher
pub trait Hasher {
    fn convert_to_bytes (&self) -> Vec<u8>;
}

// The primary way to use hashdozen
pub fn hash<'a, T: Hasher> (input: &T, salt: &T) -> Result<String, &'a str> {
    let hashinput = input.convert_to_bytes();
    let saltinput = salt.convert_to_bytes();

    if hashinput.len() == 0 || saltinput.len() == 0 {
        let ret: &'a str = "One or more inputs were empty.";
        return Result::Err(ret);
    }

    return Ok(run_hash((hashinput, saltinput)));
}