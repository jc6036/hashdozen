pub mod hashdozen;

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    #[test]
    fn try_hash_string(){
        let test1 = String::from("Test1");
        let test2 = String::from("Test2");
        hashdozen::hash(&test1, &test2);
    }
}