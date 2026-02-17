use std::fmt;

use crate::error::ParseError;

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

#[derive(Debug,PartialEq)]
pub struct RecordingOperation {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: i64,
    pub status: Status,
    pub desc_len: usize, //not in csv and txt
    pub description: String,
}

pub enum FieldRecordingOperation {
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

impl FieldRecordingOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TxId => "TX_ID",
            Self::TxType => "TX_TYPE",
            Self::FromUserId => "FROM_USER_ID",
            Self::ToUserId => "TO_USER_ID",
            Self::Amount => "AMOUNT",
            Self::Timestamp => "TIMESTAMP",
            Self::Status => "STATUS",
            Self::DescLen => "DESC_LEN",
            Self::Description => "DESCRIPTION",
        }
    }
}

impl std::fmt::Display for FieldRecordingOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TxId => write!(f, "TX_ID"),
            Self::TxType => write!(f, "TX_TYPE"),
            Self::FromUserId => write!(f, "FROM_USER_ID"),
            Self::ToUserId => write!(f, "TO_USER_ID"),
            Self::Amount => write!(f, "AMOUNT"),
            Self::Timestamp => write!(f, "TIMESTAMP"),
            Self::Status => write!(f, "STATUS"),
            Self::DescLen => write!(f, "DESC_LEN"),
            Self::Description => write!(f, "DESCRIPTION"),
        }
    }
}

impl fmt::Debug for FieldRecordingOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TxId => write!(f, "TX_ID"),
            Self::TxType => write!(f, "TX_TYPE"),
            Self::FromUserId => write!(f, "FROM_USER_ID"),
            Self::ToUserId => write!(f, "TO_USER_ID"),
            Self::Amount => write!(f, "AMOUNT"),
            Self::Timestamp => write!(f, "TIMESTAMP"),
            Self::Status => write!(f, "STATUS"),
            Self::DescLen => write!(f, "DESC_LEN"),
            Self::Description => write!(f, "DESCRIPTION"),
        }
    }
}

impl TxType {
    pub fn str_to_tx_type(value: &str) -> Result<TxType, ParseError> {
        match value {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            _ => Err(ParseError::IncorrectOperation {
                operation: value.to_string(),
            }),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
        }
    }

    pub fn from_u8(value: u8) -> Result<TxType, ParseError> {
        match value {
            0 => Ok(TxType::Deposit),
            1 => Ok(TxType::Transfer),
            2 => Ok(TxType::Withdrawal),
            _ => Err(ParseError::IncorrectOperation {
                operation: value.to_string(),
            }),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            TxType::Deposit => 0,
            TxType::Transfer => 1,
            TxType::Withdrawal => 2,
        }
    }
}

impl Status {
    pub fn str_to_status(value: &str) -> Result<Status, ParseError> {
        match value {
            "FAILURE" => Ok(Status::Failure),
            "PENDING" => Ok(Status::Pending),
            "SUCCESS" => Ok(Status::Success),
            _ => Err(ParseError::IncorrectStatus {
                status: value.to_string(),
            }),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Failure => "FAILURE",
            Status::Pending => "PENDING",
            Status::Success => "SUCCESS",
        }
    }

    pub fn from_u8(value: u8) -> Result<Status, ParseError> {
        match value {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(ParseError::IncorrectStatus {
                status: value.to_string(),
            }),
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Status::Success => 0,
            Status::Failure => 1,
            Status::Pending => 2,
        }
    }
}
