#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[derive(Debug)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

#[derive(Debug)]
pub struct RecordingOperation {
    pub tx_id : u64,
    pub tx_type : TxType,
    pub from_user_id : u64,
    pub to_user_id : u64,
    pub amount : i64,
    pub timestamp : i64,
    pub status : Status,
    pub desc_len : usize, //not in csv and txt
    pub description : String,
}

impl From<TxType> for String {
    
    fn from(value: TxType) -> Self {
        match value {
            TxType::Deposit => "DEPOSIT".to_string(),
            TxType::Transfer => "TRANSFER".to_string(),
            TxType::Withdrawal => "WITHDRAWAL".to_string(),
        }
    }
}

impl TxType{
    
    pub fn from_str(value: &str) -> Result<TxType, ()> {
        match value {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            _ => Err(())
        }
    }
}

impl From<Status> for String {
    fn from(value: Status) -> Self {
        match value {
            Status::Failure => "FAILURE".to_string(),
            Status::Pending => "PENDING".to_string(),
            Status::Success => "SUCCESS".to_string(),
        }
    }
}

impl Status{
    
    pub fn from_str(value: &str) -> Result<Status, ()> {
        match value {
            "FAILURE" => Ok(Status::Failure),
            "PENDING" => Ok(Status::Pending),
            "SUCCESS" => Ok(Status::Success),
            _ => Err(())
        }
    }
}