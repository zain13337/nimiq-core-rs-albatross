use beserial::{Serialize, Deserialize};

#[derive(Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Debug,Serialize,Deserialize)]
pub struct HashedTimeLockedContract {
    pub balance: u64
}
