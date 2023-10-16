fn login(creds: models::Credentials) {
    // authenticate..
    crate::database::get_user();
}

fn logout() {
    // logout
}

pub mod models;
