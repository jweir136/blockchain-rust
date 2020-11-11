use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Result, Formatter};

/// Transaction item is used to represent a single transaction. Holds address to send the money to, address to send the money to, and the amount
/// of money to send money to.
pub struct Transaction {
    to : String,
    from : String,
    amount : f32
}

/// Block item is used to store a collection of transactions, the hash of the transactions, the hash of the last block in the chain, and the proof
/// of work associated to the block.
pub struct Block {
    transactions : Vec<Transaction>,
    hash : u64,
    last_hash : u64,
    proof : u64
}

/// Blockchain item is a public data structure that holds the blocks and the pending transactions that are yet to be added to the blockchain.
pub struct Blockchain {
    size : usize,
    pending_transactions : Vec<Transaction>,
    blocks : Vec<Block>
}

impl Hash for Transaction {
    /// Compute the hash of a transaction using the information included in the Transaction item.
    /// The hash uses the dollars and cents of the amount since the f32 data type is not hashable.
    fn hash<H: Hasher>(&self, state: &mut H) {
        let dollars : u32 = self.amount as u32;
        let cents : u32 = ((self.amount - dollars as f32) * 100f32) as u32;

        self.to.hash(state);
        self.from.hash(state);
        dollars.hash(state);
        cents.hash(state);
    }
}

impl Hash for Block {
    /// Compute the hash of a block using all the elements in each transaction.
    fn hash<H : Hasher>(&self, state : &mut H) {
        let mut dollars : u32;
        let mut cents : u32;

        for transaction in &self.transactions {
            dollars = transaction.amount as u32;
            cents = ((transaction.amount - dollars as f32) * 100f32) as u32;

            transaction.to.hash(state);
            transaction.from.hash(state);
            dollars.hash(state);
            cents.hash(state);
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "to: {}, from: {}, amount: {}", self.to, self.from, self.amount)
    }
}

impl Display for Block {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "Hash: {}, Last Hash: {}, proof: {}, transactions size: {}", self.hash, self.last_hash, self.proof, self.transactions.len())
    }
}

impl Transaction {
    /// Create and return a new Transaction item using a given 'to' address, 'from' address, and a monetary amount.
    pub fn new(to : String, from : String, amount : f32) -> Self {
        Transaction {
            to : to,
            from : from,
            amount : amount
        }
    }

    /// static method that computes the hash of a Transaction item using the DefaultHasher collection object.
    pub fn calculate_hash<T : Hash>(t : &T) -> u64 {
        let mut s  = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

impl Block {
    /// Creates and returns a new Block item using the given list of transactions and the last hash in the chain.
    /// Upon being created, the proof of work is computed and the hash of the entire block is calculated.
    /// NOTE: The computation of the proof of work may take a while (that's the whole point of the proof of work).
    pub fn new(transactions : Vec<Transaction>, last_hash : u64) -> Self {
        Block {
            proof : Block::calculate_proof_of_work(last_hash),
            hash : Block::calculate_hash(&transactions),
            transactions : transactions,
            last_hash : last_hash
        }
    }

    pub fn calculate_hash<T : Hash>(t : &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    /// Determines if the proof proposed by the block is valid or not. The criteria is that the last 'N' digits of the proof + the last hash ends in
    /// at least 6 '0' characters.
    fn is_proof_valid(proof : u64) -> bool {
        let proof_string = &proof.to_string();

        for chr in proof_string.chars().rev().take(6) {
            if chr != '0' {
                return false;
            }
        }

        true
    }

    /// Calculates and returns the hash of a String value using the DefaultHasher item.
    fn calculate_string_hash(x : &String) -> u64 {
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    }

    /// Method used to find the proof of work. The proof starts at 0 and iteritively goes increments the value of the proof until the criteria is true.
    fn calculate_proof_of_work(last_hash : u64) -> u64{
        let mut proof : u64 = 0;
        let mut string_proof : String = last_hash.to_string() + &proof.to_string();
        let mut hash : u64 = Block::calculate_string_hash(&string_proof);
        
        while !Block::is_proof_valid(hash) {
            proof += 1;
            string_proof = last_hash.to_string() + &proof.to_string();
            hash = Block::calculate_string_hash(&string_proof);
        }

        hash
    }
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain { size : 0, pending_transactions : Vec::<Transaction>::new(), blocks : Vec::<Block>::new() }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn add_transaction(&mut self, transaction : Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn add_block(&mut self) {
        let last_hash : u64 = Block::calculate_hash(self.blocks.last().unwrap());
    }
}
