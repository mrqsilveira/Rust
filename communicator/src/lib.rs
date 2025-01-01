pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]


mod tests {
    //use super::*;
    use super::client;
    #[test]
    fn it_works() {
        //let result = crate::add(2, 2);
        let result = add(2, 2);
        assert_eq!(result, 4);

        //crate::client::connect();
        client::connect();
    }
}

pub mod client;

pub mod network;


