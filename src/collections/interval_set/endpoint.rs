#[derive(Debug, Clone, PartialEq)]
pub enum Endpoint<T: PartialOrd + Clone> {
    Open(T),
    Closed(T),
    Unbounded,
}
