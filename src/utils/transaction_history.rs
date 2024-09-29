use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::Mutex;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction;

pub static TRANSACTION_HISTORY: Lazy<Mutex<VecDeque<SubscribeUpdateTransaction>>> =
    Lazy::new(|| Mutex::new(VecDeque::with_capacity(5)));

pub fn add_transaction_to_history(transaction: SubscribeUpdateTransaction) {
    let mut history = TRANSACTION_HISTORY.lock().unwrap();
    if history.len() >= 5 {
        history.pop_back();
    }
    history.push_front(transaction);
}

pub fn get_transaction_history() -> Vec<SubscribeUpdateTransaction> {
    let history = TRANSACTION_HISTORY.lock().unwrap();
    history.iter().cloned().collect()
}
