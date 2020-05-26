pub struct AsyncLog {
    current: Arc<Mutex<Vec<Oid>>>,
    sender: Sender<AsyncNotification>,
    pending: Arc<AtomicBool>,
    background: Arc<AtomicBool>,
}