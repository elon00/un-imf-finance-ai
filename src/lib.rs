use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 5,
        agency_name: "International Monetary Fund (IMF)".to_string(),
        project_name: "IMF SDRNet AI".to_string(),
        portfolio: "International Monetary System & Reserves".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Multilateral Sovereign Debt Restructuring Escrow".to_string(),
        target_chain: "Internet Computer (ICP) + Base L2".to_string(),
        is_active: true,
    }
}
