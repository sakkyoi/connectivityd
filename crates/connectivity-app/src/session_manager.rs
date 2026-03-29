use std::{collections::HashMap, sync::Mutex};

use connectivity_domain::session::{SessionId, SessionInfo};

pub struct SessionManager {
    inner: Mutex<HashMap<SessionId, SessionInfo>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, session: SessionInfo) {
        self.inner
            .lock()
            .expect("session mutex poisoned")
            .insert(session.id.clone(), session);
    }

    pub fn unregister(&self, session: SessionId) {
        self.inner
            .lock()
            .expect("session mutex poisoned")
            .remove(&session);
    }
}
