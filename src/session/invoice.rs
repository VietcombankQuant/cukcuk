use crate::{
    model::{Branch, Invoice, InvoicePagingParam, InvoiceSummary, ServiceResult},
    session::{LoginSession, API_DOMAIN},
};

impl LoginSession {
    pub async fn get_invoice_paging(
        &self,
        branch: &Branch,
        page: u32,
        limit: u32,
        last_sync_date: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Vec<InvoiceSummary>> {
        let url = format!("https://{}/api/v1/sainvoices/paging", API_DOMAIN);
        let params = InvoicePagingParam {
            page,
            limit,
            branch_id: branch.id.clone(),
            have_customer: true,
            last_sync_date: last_sync_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        };
        let resp = self.api_client.post(url).json(&params).send().await?;
        let message = resp.text().await?;
        let results: ServiceResult<Vec<InvoiceSummary>> = serde_json::from_str(&message)?;

        if !results.success {
            return Err(anyhow::anyhow!(
                "Failed to get invoice paging for branch {} with error code {} - error message {}",
                branch.name,
                results.error_type.unwrap_or_default(),
                results.error_message.unwrap_or_default()
            ));
        }

        let invoices = results.data.unwrap_or_default();
        Ok(invoices)
    }

    pub async fn get_invoice(&self, invoice_ref: &str) -> anyhow::Result<Invoice> {
        let url = format!("https://{}/api/v1/sainvoices/{}", API_DOMAIN, invoice_ref);
        let resp = self.api_client.get(url).send().await?;
        let message = resp.text().await?;
        println!("message:\n{}", message);
        let results: ServiceResult<Invoice> = serde_json::from_str(&message)?;
        if !results.success {
            return Err(anyhow::anyhow!(
                "Failed to get invoice detail for invoice {} with error code {} - error message {}",
                invoice_ref,
                results.error_type.unwrap_or_default(),
                results.error_message.unwrap_or_default()
            ));
        }

        let invoice_details = results.data.unwrap_or_default();
        Ok(invoice_details)
    }
}
