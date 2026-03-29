use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use connectivity_domain::session::{SessionId, SessionInfo};

pub struct SessionManager {
    sessions: RwLock<HashMap<SessionId, SessionInfo>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, session: SessionInfo) {
        self.sessions
            .write()
            .expect("session registry poisoned")
            .insert(session.id.clone(), session);
    }

    pub fn unregister(&self, session_id: &SessionId) -> Option<SessionInfo> {
        self.sessions
            .write()
            .expect("session registry poisoned")
            .remove(session_id)
    }

    pub fn get(&self, session_id: &SessionId) -> Option<SessionInfo> {
        self.sessions
            .read()
            .expect("session registry poisoned")
            .get(session_id)
            .cloned()
    }

    pub fn list(&self) -> Vec<SessionInfo> {
        self.sessions
            .read()
            .expect("session registry poisoned")
            .values()
            .cloned()
            .collect()
    }
}

pub type SharedSessionManager = Arc<SessionManager>;
