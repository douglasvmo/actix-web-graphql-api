use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BearerToken {
  pub jwt: Option<Claims>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Claims {
    
}