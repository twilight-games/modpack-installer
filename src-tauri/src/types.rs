use serde::{Serialize};



    #[derive(Serialize, Clone)]
    pub struct ProgressPayload {
      pub bytes: u64,
    }

    #[derive(Serialize, Clone)]
    pub struct TotalFileSizePayload {
      pub total_bytes: u64,
    }
    
    #[derive(Serialize, Clone)]
    pub struct FinishedPayload {
      pub finished: bool,
      pub errors: bool,
    }
    #[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
    pub struct Modpack {
      pub id: String,
      pub name: String,
      pub description: String,
      pub version: String
    }

