pub mod cassette {
    pub fn eq(exp:u8, i:u8) -> bool{
        (exp&(1<<i)) != 0
    }
}