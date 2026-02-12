use std::fmt;

use crate::error::ParseError;

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

pub enum FieldRecordingOperation{
    TxId,
    TxType,
    FromUserId,
    ToUserId,
    Amount,
    Timestamp,
    Status,
    DescLen,
    Description,
}

impl fmt::Debug for FieldRecordingOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TxId => write!(f, "TX_ID"),
            Self::TxType => write!(f, "TX_TYPE"),
            Self::FromUserId => write!(f, "FROM_USER_ID"),
            Self::ToUserId => write!(f, "TO_USER_ID"),
            Self::Amount => write!(f, "AMOUNT"),
            Self::Timestamp => write!(f, "TimesTIMESTAMPtamp"),
            Self::Status => write!(f, "STATUS"),
            Self::DescLen => write!(f, "DESC_LEN"),
            Self::Description => write!(f, "DESCRIPTION"),
        }
    }
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
    
    pub fn str_to_tx_type(value: &str) -> Result<TxType, ParseError> {
        match value {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            _ => Err(ParseError::IncorrectOperation { operation: value.to_string() })
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
    
    pub fn str_to_status(value: &str) -> Result<Status, ParseError> {
        match value {
            "FAILURE" => Ok(Status::Failure),
            "PENDING" => Ok(Status::Pending),
            "SUCCESS" => Ok(Status::Success),
            _ => Err(ParseError::IncorrectStatus { status: value.to_string() })
        }
    }
}