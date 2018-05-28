// mod is the module keyword

// pub is the keyword to make module code 
// public. By default, module code is private

// use is the keyword to bring module code
// into the current scope for easy use

// mod client {
//     fn connect() {
        
//     }
// }

// mod network {
//     fn connect() {

//     }

//     mod server {
//         fn connect() {
            
//         }
//     }
// }

// module declarations with only a name and an
// ending semicolon refer to code in a matching 
// adjacent filename

pub mod client;
pub mod network;


#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // start from root crate
        // ::client::connect();
        // start one module up
        // super::client::connect();
        client::connect();
        //assert_eq!(2 + 2, 4);
    }
}
