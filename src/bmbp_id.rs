use uuid::{Uuid};

pub struct BmbpId;

impl BmbpId {
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }
    pub fn simple_uuid() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }
}

#[cfg(test)]
mod tests {
    use crate::BmbpId;

    #[test]
    fn test_uuid() {
        assert_eq!(BmbpId::uuid().len(), 36);
        assert_eq!(BmbpId::simple_uuid().len(), 32);
    }
}