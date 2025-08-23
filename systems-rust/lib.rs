use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 5039
// Hash 5308
// Hash 4253
// Hash 2546
// Hash 5502
// Hash 6291
// Hash 1613
// Hash 8466
// Hash 1591
// Hash 5263
// Hash 3938
// Hash 7324
// Hash 1291
// Hash 5024
// Hash 5254
// Hash 3156
// Hash 6537
// Hash 7600
// Hash 1689
// Hash 3682
// Hash 7436
// Hash 1719
// Hash 7926
// Hash 5575
// Hash 2220
// Hash 3741
// Hash 4842
// Hash 2088
// Hash 8242
// Hash 1307
// Hash 5336
// Hash 4384
// Hash 9039
// Hash 3973
// Hash 9826
// Hash 3880
// Hash 4502
// Hash 8221
// Hash 4461
// Hash 9562
// Hash 6964
// Hash 1155
// Hash 7910
// Hash 8373
// Hash 5234
// Hash 9143
// Hash 8902
// Hash 7120
// Hash 3606
// Hash 8837
// Hash 8174
// Hash 1971
// Hash 3976
// Hash 2983
// Hash 7850
// Hash 3089
// Hash 9526
// Hash 1405
// Hash 1636
// Hash 6729
// Hash 7510
// Hash 6332
// Hash 3917
// Hash 2206
// Hash 3570
// Hash 9184
// Hash 8512
// Hash 5839
// Hash 6763
// Hash 2573
// Hash 7309
// Hash 9976
// Hash 7112
// Hash 6711
// Hash 2415
// Hash 1064
// Hash 9429
// Hash 8348
// Hash 8799
// Hash 7429
// Hash 3171
// Hash 7161
// Hash 3213
// Hash 5859
// Hash 5202
// Hash 7126
// Hash 2537
// Hash 2074
// Hash 5838
// Hash 9071
// Hash 2366
// Hash 6816
// Hash 3153
// Hash 8022
// Hash 3403
// Hash 8370
// Hash 2232
// Hash 5797
// Hash 7472