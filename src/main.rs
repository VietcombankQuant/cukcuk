use ::cukcuk::{model::LoginParam, session::LoginSession, utils};

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
    let active_date = utils::date_from_parts(2023, 5, 9);

    for branch in branches {
        let invoices = session.get_invoices(&branch, active_date).await?;
        for invoice in invoices {
            println!("{:#?}", invoice);
        }
    }

    Ok(())
}
