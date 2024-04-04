/// Hashdozen: A novel 48 bit hashing algorithm designed to produce a human-readable output with minimal collision and high speed.
/// 
/// This project started as a way to dig into Rust after I read the Rust Book, but it ended up having real applications.
/// Primarily, the intention is to provide a unique identifier of a dozen characters, including 3 dashes, that takes up
/// minimal space and is human readable, able to take various identifying bits of information to produce and imminently searchable in
/// a table.
/// 
/// Uniqueness is good in keys of greater than 6 bytes. Below 6 bytes, you will run into collisions relatively frequently. Salting
/// keys less than 6 bytes helps but still provides little gurantee.
/// 
/// Hashdozen includes a built-in salt generator that will double the size of a key and make it much more unique. This salting
/// isn't intended for security (though it could be utilized towards that end) but to ensure incoming keys are more unique.
/// Left separate from the main hash in case the user has a faster or more relevant salt they'd like to use.
/// 
/// For usage, please review the publically exposed methods. You may fork or submit pull requests on github, or send any issue
/// mail to jc6036@gmail.com.
pub mod hashdozen;

/* API Tests */
#[cfg(test)]
mod tests {
    use std::string::String;
    use super::*;

    // Ensure the salt adds bytes to an input
    #[test]
    fn salted_value_greater_than_input() {
        let data_vec = Vec::<u8>::from(String::from("TestData"));
        let data_str_size = data_vec.len();

        let salted_data = hashdozen::generate_salted_input(data_vec);
        let salt_size = salted_data.len();

        assert!(salt_size > data_str_size);
    }

    // Ensure the salted value is different from the data
    #[test]
    fn salted_value_different () {
        let data_vec = Vec::<u8>::from(String::from("TestData"));
        let salted_vec = hashdozen::generate_salted_input(data_vec.clone());

        assert_ne!(data_vec, salted_vec);
    }

    // Ensure that if you use the same data twice, you get the same salted data twice
    #[test]
    fn salt_equivalence () {
        let data_vec_1 = Vec::<u8>::from(String::from("TestData1"));
        let data_vec_2 = data_vec_1.clone();

        let salted_data_1 = hashdozen::generate_salted_input(data_vec_1.clone());
        let salted_data_2 = hashdozen::generate_salted_input(data_vec_2.clone());

        assert_eq!(salted_data_1, salted_data_2);
    }

    // Ensure that a hash will produce a unique value for two keys of 6 bytes if at least 1 byte is different
    #[test]
    fn hash_simple_uniqueness() {
        let data_vec_1 = Vec::<u8>::from(String::from("FOOBAR"));
        let data_vec_2 = Vec::<u8>::from(String::from("FOOBAZ"));

        let hashed_vec_1 = hashdozen::hash(data_vec_1);
        let hashed_vec_2 = hashdozen::hash(data_vec_2);

        assert_ne!(hashed_vec_1, hashed_vec_2);
    }

    // Ensure that the same 6 byte key produces the same hash
    #[test]
    fn hash_simple_reproduction() {
        let data_vec_1 = Vec::<u8>::from(String::from("FOOBAR"));
        let data_vec_2 = Vec::<u8>::from(String::from("FOOBAR"));

        let hashed_vec_1 = hashdozen::hash(data_vec_1);
        let hashed_vec_2 = hashdozen::hash(data_vec_2);

        assert_eq!(hashed_vec_1, hashed_vec_2);
    }

    // Ensure the hashed data != the input data
    #[test]
    fn hash_changes_input() {
        let comp_str = String::from("TestString");
        let data_vec = Vec::<u8>::from(comp_str.clone());
        let hashed_vec = hashdozen::hash(data_vec);

        assert_ne!(comp_str, hashed_vec);
    }

    // Ensure that 2 salted values generate the same hash
    #[test]
    fn salted_hash_reproduction() {
        let data_vec_1 = Vec::<u8>::from(String::from("FOOBAR"));
        let data_vec_2 = Vec::<u8>::from(String::from("FOOBAR"));

        let salty_data_1 = hashdozen::generate_salted_input(data_vec_1);
        let salty_data_2 = hashdozen::generate_salted_input(data_vec_2);

        let hashed_vec_1 = hashdozen::hash(salty_data_1);
        let hashed_vec_2 = hashdozen::hash(salty_data_2);

        assert_eq!(hashed_vec_1, hashed_vec_2);      
    }
}