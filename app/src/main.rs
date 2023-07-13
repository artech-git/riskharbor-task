mod constants;



#[tokio::main]
async fn main() -> Result<(), BackendError> {

    let mut db_handler = Database::init(constants::DbUrl).unwrap();

    let mut consensus_data = 

    let mut slotIndexer = SlotIndexer::new();


    let db_handler = Arc::new();

    let app = Router::new()
        .route("/v1/:validator/participation", get(get_validator_participation))
        .route("/v1/network/participation", get(get_network_participation));

    #[cfg(debug_assertions)] // select the following block if the --release flag is not present
    {
        axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
            .http2_enable_connect_protocol() //enable http2 connection procedure for the axum server
            .serve(app.into_make_service()) //serve our application on this route
            .await
            .unwrap();
    }

}