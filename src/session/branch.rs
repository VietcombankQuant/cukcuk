use futures::{StreamExt, TryStreamExt};

use crate::{
    model::{Branch, BranchSummary, LoginParam, LoginResponse, ServiceResult},
    session::{LoginSession, API_DOMAIN},
};

impl LoginSession {
    pub async fn get_branches(&self) -> anyhow::Result<Vec<Branch>> {
        let summaries = self.get_branch_summaries().await?;
        let branches = futures::stream::iter(&summaries)
            .map(|summary| self.get_branch_detail(&summary.id))
            .buffer_unordered(8)
            .try_collect::<Vec<Branch>>()
            .await?;
        Ok(branches)
    }

    async fn get_branch_summaries(&self) -> anyhow::Result<Vec<BranchSummary>> {
        let branch_url = format!("https://{}/api/v1/branchs/all", API_DOMAIN);
        let resp = self.api_client.get(branch_url).send().await?;
        let message = resp.text().await?;
        let result: ServiceResult<Vec<BranchSummary>> = serde_json::from_str(&message)?;
        let branches = result.data.unwrap_or_default();
        Ok(branches)
    }

    async fn get_branch_detail(&self, branch_id: &str) -> anyhow::Result<Branch> {
        const API_DOMAIN: &str = "graphapi.cukcuk.vn";
        let branch_detail_url = format!(
            "https://{}/api/v1/branchs/setting/{}",
            API_DOMAIN, branch_id
        );

        let resp = self.api_client.get(&branch_detail_url).send().await?;
        let message = resp.text().await?;
        let result: ServiceResult<Branch> = serde_json::from_str(&message)?;
        let branch = result.data.unwrap_or(Default::default());
        Ok(branch)
    }
}
