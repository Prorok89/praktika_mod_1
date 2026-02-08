pub enum Status {
    Success,
    Failure,
    Pending,
}

pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

pub struct RecordingOperation {
    tx_id : u64,
    tx_type : String,
    from_user_id : u64,
    to_user_id : u64,
    amount : i64,
    timestamp : i64,
    status : Status,
    desc_len : i64, //not in csv and txt
    description : String,
}