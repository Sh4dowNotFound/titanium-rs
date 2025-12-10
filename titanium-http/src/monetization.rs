use crate::error::HttpError;
use crate::HttpClient;
use titanium_model::{Entitlement, Sku, Snowflake};

impl HttpClient {
    /// List SKUs for an application.
    pub async fn list_skus(&self, application_id: Snowflake) -> Result<Vec<Sku>, HttpError> {
        self.get(&format!("/applications/{}/skus", application_id))
            .await
    }

    /// List entitlements for an application.
    pub async fn list_entitlements(
        &self,
        application_id: Snowflake,
        query: Option<&serde_json::Value>,
    ) -> Result<Vec<Entitlement>, HttpError> {
        let route = format!("/applications/{}/entitlements", application_id);

        if let Some(q) = query {
            self.get_with_query(&route, q).await
        } else {
            self.get(&route).await
        }
    }

    /// Get an entitlement.
    pub async fn get_entitlement(
        &self,
        application_id: Snowflake,
        entitlement_id: Snowflake,
    ) -> Result<Entitlement, HttpError> {
        self.get(&format!(
            "/applications/{}/entitlements/{}",
            application_id, entitlement_id
        ))
        .await
    }

    /// Create a test entitlement.
    pub async fn create_test_entitlement(
        &self,
        application_id: Snowflake,
        payload: &serde_json::Value,
    ) -> Result<Entitlement, HttpError> {
        self.post(
            &format!("/applications/{}/entitlements", application_id),
            payload,
        )
        .await
    }

    /// Delete a test entitlement.
    pub async fn delete_test_entitlement(
        &self,
        application_id: Snowflake,
        entitlement_id: Snowflake,
    ) -> Result<(), HttpError> {
        self.delete(&format!(
            "/applications/{}/entitlements/{}",
            application_id, entitlement_id
        ))
        .await
    }
}
