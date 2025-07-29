use crate::config::{STORAGE_USER_API_KEY, STORAGE_USER_EMAIL};
use crate::errors::{AppError, AppResult};
use crate::types::UserData;

pub struct StorageService;

impl StorageService {
    pub fn save_user_data(user_data: &UserData) -> AppResult<()> {
        let storage = Self::get_storage()?;

        storage
            .set_item(STORAGE_USER_EMAIL, &user_data.email)
            .map_err(|e| AppError::storage(format!("Failed to save email: {:?}", e)))?;

        storage
            .set_item(STORAGE_USER_API_KEY, &user_data.api_key)
            .map_err(|e| AppError::storage(format!("Failed to save API key: {:?}", e)))?;

        Ok(())
    }

    pub fn load_user_data() -> AppResult<Option<UserData>> {
        let storage = Self::get_storage()?;

        let email = storage
            .get_item(STORAGE_USER_EMAIL)
            .map_err(|e| AppError::storage(format!("Failed to load email: {:?}", e)))?;

        let api_key = storage
            .get_item(STORAGE_USER_API_KEY)
            .map_err(|e| AppError::storage(format!("Failed to load API key: {:?}", e)))?;

        match (email, api_key) {
            (Some(email), Some(api_key)) if !email.is_empty() && !api_key.is_empty() => {
                Ok(Some(UserData::new(email, api_key)))
            }
            _ => Ok(None),
        }
    }

    pub fn clear_user_data() -> AppResult<()> {
        let storage = Self::get_storage()?;

        storage
            .remove_item(STORAGE_USER_EMAIL)
            .map_err(|e| AppError::storage(format!("Failed to remove email: {:?}", e)))?;

        storage
            .remove_item(STORAGE_USER_API_KEY)
            .map_err(|e| AppError::storage(format!("Failed to remove API key: {:?}", e)))?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn clear_all() -> AppResult<()> {
        let storage = Self::get_storage()?;

        storage
            .clear()
            .map_err(|e| AppError::storage(format!("Failed to clear storage: {:?}", e)))?;

        Ok(())
    }

    fn get_storage() -> AppResult<web_sys::Storage> {
        let window = web_sys::window().ok_or_else(|| AppError::storage("Window not available"))?;

        let storage = window
            .local_storage()
            .map_err(|e| AppError::storage(format!("Failed to access storage: {:?}", e)))?
            .ok_or_else(|| AppError::storage("Local storage not available"))?;

        Ok(storage)
    }

    #[allow(dead_code)]
    pub fn is_available() -> bool {
        Self::get_storage().is_ok()
    }
}
