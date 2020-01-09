Day 72: Hash Table — open addressing 
<br>
Like separate chaining, open addressing is a method for handling collisions. In Open Addressing, all elements are stored in the hash table itself. So at any point, size of the table must be greater than or equal to the total number of keys (Note that we can increase table size by copying old data if needed).
<br>
Insert(k): Keep probing until an empty slot is found. Once an empty slot is found, insert k.
<br>
Search(k): Keep probing until slot’s key doesn’t become equal to k or an empty slot is reached.
<br>
Delete(k): Delete operation is interesting. If we simply delete a key, then search may fail. So slots of deleted keys are marked specially as “deleted”.
Insert can insert an item in a deleted slot, but the search doesn’t stop at a deleted slot.
<br>
Open Addressing is done by linear probing:
<br>
Linear Probing: In linear probing, we linearly probe for next slot. For example, typical gap between two probes is 1 as taken in below example also.
let hash(x) be the slot index computed using hash function and S be the table size
```
If slot hash(x) % S is full, then we try (hash(x) + 1) % S
If (hash(x) + 1) % S is also full, then we try (hash(x) + 2) % S
If (hash(x) + 2) % S is also full, then we try (hash(x) + 3) % S 
..................................................
..................................................
```

<br>
Clustering: The main problem with linear probing is clustering, many consecutive elements form groups and it starts taking time to find a free slot or to search an element.

run:
```

fn main() {
    let mut rng = rand::thread_rng();
    let mut hash_table: HashTable = HashTable::default();
    for _i in 0..=1000 {
        let r: i32 = rng.gen_range(0, 1000);
        let chance: f64 = rng.gen();
        let node_value = format!("{}", r);
        let node = node!(node_value);
        if chance >= 0.5 {
            hash_table.insert(node);
        } else {
            hash_table.delete(node);
        }
        println!("{} {}", hash_table.count, hash_table.size);
    }
}

```

result: println!("{} {}", hash_table.count, hash_table.size);
```
411 1024
```