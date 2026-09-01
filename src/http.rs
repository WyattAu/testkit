use axum::Router;

/// A test HTTP server that starts on a random port.
pub struct TestServer {
    addr: String,
    handle: tokio::task::JoinHandle<()>,
}

impl TestServer {
    /// Create a new test server from an Axum router.
    pub async fn new(app: Router) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind");
        let addr = listener.local_addr().expect("failed to get addr").to_string();

        let handle = tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .expect("server failed");
        });

        Self { addr, handle }
    }

    /// Get the base URL of the test server.
    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// Get the full base URL (with http:// prefix).
    pub fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Stop the test server.
    pub async fn stop(self) {
        self.handle.abort();
    }
}
