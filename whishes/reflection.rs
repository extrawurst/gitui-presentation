fn is_int<T>(t:T) -> bool{
    static if T : i32 {
        true
    } else {
        false
    }
}
