use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    _credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // TODO: fill in validator logic
    log::debug!("Authentication validation triggered");
    Ok(req)
    // if credentials.token() == "mF_9.B5f-4.1JqM" {
    //     Ok(req)
    // } else {
    //     let config = req
    //         .app_data::<bearer::Config>()
    //         .cloned()
    //         .unwrap_or_default()
    //         .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

    //     Err((AuthenticationError::from(config).into(), req))
    // }
}
