Rust library to generate 12 character hashes out of arbitrary amounts of data. Intended to be used for generation of unique values for things like profile URL tags. 

NOT SECURE.

Extensible - implement the Hasher trait to tell it how to convert your type to a vec of u8, and you should be good. Always returns a String containg the hex representation of the final data.
