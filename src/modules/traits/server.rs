pub trait Server {
    fn handle_request(&self, data: &[u8]) -> Vec<u8>;
    fn start(&self);
}
