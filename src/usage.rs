use serde::Deserialize;

/// Response of the `translation` call.
#[derive(Clone, Debug, Deserialize)]
pub struct Usage {
    /// `character_count`.
    pub character_count: Option<u64>,
    /// `character_limit`.
    pub character_limit: Option<u64>,
    /// `document_limit`.
    pub document_limit: Option<u64>,
    /// `document_count`.
    pub document_count: Option<u64>,
    /// `team_document_limit`.
    pub team_document_limit: Option<u64>,
    /// `team_document_count`.
    pub team_document_count: Option<u64>,
}

impl Usage {
    /// Increase the `character_count` and check if it's within `character_limit`.
    ///
    /// * If `character_count` is None, false is returned.
    /// * `character_count` is increased by `additional`
    /// * If `character_limit` is None, false is returned.
    /// * It's returned whether `character_count` is less or equal than `character_limit`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `additional` can't be converted into a `u64`.
    pub fn add<T: TryInto<u64>>(
        &mut self,
        additional: T,
    ) -> Result<bool, <T as TryInto<u64>>::Error> {
        let additional = additional.try_into()?;
        if let Some(character_count) = &mut self.character_count {
            *character_count = character_count.saturating_add(additional);
            if let Some(character_limit) = self.character_limit {
                Ok(*character_count <= character_limit)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Check if `character_count+additional` is within `character_limit`.
    ///
    /// * If `character_count` is None, false is returned.
    /// * If `character_limit` is None, false is returned.
    /// * It's returned whether `character_count+additional` is less or equal than `character_limit`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `additional` can't be converted into a `u64`.
    pub fn check<T: TryInto<u64>>(
        &self,
        additional: T,
    ) -> Result<bool, <T as TryInto<u64>>::Error> {
        let additional = additional.try_into()?;
        if let Some(mut character_count) = self.character_count {
            character_count = character_count.saturating_add(additional);
            if let Some(character_limit) = self.character_limit {
                Ok(character_count <= character_limit)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}
