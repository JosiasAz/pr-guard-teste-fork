use voidgate_backend::{app, AppState};

#[test]
fn app_has_expected_signature() {
    let _build: fn(AppState) -> axum::Router = app;
}
