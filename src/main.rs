use ::cukcuk::{model::LoginParam, session::LoginSession};
use chrono::{Datelike, Timelike};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get access token
    let login_param = LoginParam::new(
        "bownb",
        "CUKCUKOpenPlatform",
        "cd76e8fe462e8af36d137cd78a522c2d83bbd0861f2905e9b3180165c0d53df0",
    );

    let session = LoginSession::new(&login_param).await?;

    // Get branches
    let branches = session.get_branches().await?;
    let active_date = {
        let date = chrono::DateTime::<chrono::Utc>::default();
        let date = date.with_day(9).unwrap_or_default();
        let date = date.with_month(5).unwrap_or_default();
        let date = date.with_year(2023).unwrap_or_default();
        let date = date.with_hour(0).unwrap_or_default();
        let date = date.with_minute(0).unwrap_or_default();
        let date = date.with_second(0).unwrap_or_default();
        date
    };

    for branch in branches {
        let summaries = session
            .get_invoice_paging(&branch, 1, 1, active_date)
            .await?;

        for summary in summaries {
            let invoice = session.get_invoice(&summary.ref_id).await?;
            println!("{:#?}", invoice);
        }
    }

    Ok(())
}
