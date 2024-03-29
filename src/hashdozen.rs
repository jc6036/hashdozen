// Also: Tests, documentation comments, work out how to structure package for crates.io
// Todo: Update to minimize copying, minimize method calls

// Primary Hasher Implementation
fn run_hash (mut input: (Vec<u8>, Vec<u8>)) -> String {
    // Pad out to multiple of 6 bytes
    pad (&mut input, &(255, 255));

    // Compress each vec into one block
    let datablocks = compress(input);

    // Finally, combine the main data block and the salt
    let finaldata = combine(&datablocks);

    println!("{:x?}",finaldata);

    // Now output as a string
    let mut outstr: String = String::new();

    for i in finaldata.iter().enumerate() {
        match i.0 + 1 == 2 || i.0 + 1 == 4 {
            true => outstr = format!("{outstr}{:02X}-",i.1),
            false => outstr = format!("{outstr}{:02X}",i.1),
        }
    }

    return outstr;
}

fn pad(data: &mut (Vec<u8>, Vec<u8>), padvals: &(u8, u8)) {
    while data.0.len() % 6 > 0 {
        data.0.push(padvals.0);
    }

    while data.1.len() % 6 > 0 {
        data.1.push(padvals.1);
    }
}

fn compress(data: (Vec<u8>, Vec<u8>)) -> (Vec<u8>, Vec<u8>) {
    let mut main = data.0;
    let mut salt = data.1;

    let mut mainblock: Vec<u8> = main[main.len() - 6..].to_vec();
    let mut saltblock: Vec<u8> = salt[salt.len() - 6..].to_vec();

    main.truncate(main.len() - 6);

    salt.truncate(salt.len() - 6);
    while main.len() >= 6 {
        // XOR the first 6 bytes of data into our datablock
        mainblock = mainblock
                        .iter()
                        .zip(main[main.len() - 6..].iter())
                        .map(|(&x1, &x2)| x1 ^ x2)
                        .collect();
        main.truncate(main.len() - 6);
    }
    while salt.len() >= 6 {
        saltblock = saltblock
                        .iter()
                        .zip(salt[salt.len() - 6..].iter())
                        .map(|(&x1, &x2)| x1 ^ x2)
                        .collect();
        salt.truncate(salt.len() - 6);
    }

    return (mainblock, saltblock);
}

fn combine (data: &(Vec<u8>, Vec<u8>)) -> Vec<u8> {
    data.0
        .iter()
        .zip(data.1.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect()
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
pub fn hash<T: Hasher> (input: &T, salt: &T) -> String {
    let hashinput = input.convert_to_bytes();
    let saltinput = salt.convert_to_bytes();

    return run_hash((hashinput, saltinput));
}