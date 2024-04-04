Hashdozen: A novel 48 bit hashing algorithm designed to produce a human-readable output with minimal collision and high speed.

This project started as a way to dig into Rust after I read the Rust Book, but it ended up having real applications.
Primarily, the intention is to provide a unique identifier of a dozen characters, including 3 dashes, that takes up
minimal space and is human readable, able to take various identifying bits of information to produce and imminently searchable in
a table.

Uniqueness is good in keys of greater than 6 bytes. Below 6 bytes, you will run into collisions relatively frequently. Salting
keys less than 6 bytes helps but still provides little gurantee.

Hashdozen includes a built-in salt generator that will double the size of a key and make it much more unique. This salting
isn't intended for security (though it could be utilized towards that end) but to ensure incoming keys are more unique.
Left separate from the main hash in case the user has a faster or more relevant salt they'd like to use.

For usage, please review the publically exposed methods. You may fork or submit pull requests on github, or send any issue
mail to jc6036@gmail.com.