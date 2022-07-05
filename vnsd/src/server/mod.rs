pub mod api;
pub mod http;

use api::{auth::Auth, services};
use app;

use std::{
    error::Error as ErrorTrait,
    io::Result as IOResult,
    pin::Pin,
    string::ToString,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use actix_server::{Server as ActixServer, ServerHandle as ActixServerHandle};
use actix_web::{
    middleware::Logger,
    web::{self, route},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
#[derive(Clone)]
pub struct ServerHandlingError {
    cause: String,
    kind: ServerHandlingErrorKind,
}

impl ServerHandlingError {
    pub fn new(kind: ServerHandlingErrorKind, cause: &str) -> Self {
        Self {
            kind,
            cause: cause.to_owned(),
        }
    }

    pub fn cause(&self) -> String {
        self.cause.clone()
    }
    pub fn kind(&self) -> ServerHandlingErrorKind {
        self.kind
    }
}

impl std::fmt::Display for ServerHandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ServerHandlingErrorKind {
    ServerAlreadyPause,
    ServerAlreadyRunning,
    ServerAlreadyStopped,
    ServerStopped,
}

impl std::fmt::Display for ServerHandlingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::ServerHandlingErrorKind::*;
        match *self {
            ServerAlreadyPause => write!(f, "server already paused"),
            ServerAlreadyRunning => write!(f, "server already running"),
            ServerAlreadyStopped => write!(f, "server already stopped"),
            ServerStopped => write!(f, "server was stopped"),
        }
    }
}

impl ErrorTrait for ServerHandlingErrorKind {
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match *self {
            _ => None,
        }
    }
}

type ActixServerRunner = Arc<Mutex<Pin<Box<ActixServer>>>>;
#[derive(Clone)]
pub struct Server {
    addr: ServerAddr,
    runner: ActixServerRunner,
    handler: Arc<ActixServerHandle>,
    status: ServerStatus,
}

impl Server {
    pub fn new() -> IOResult<Self> {
        let addr = ServerAddr::from_config_file();
        let runner = match ServerRunner::new(addr.clone()) {
            Err(e) => return Err(e),
            Ok(s) => s,
        };
        // let handler = runner.handl / e();
        Ok(Self {
            addr,
            handler: Arc::new(runner.handle()),
            runner: Arc::new(Mutex::new(Box::pin(runner))),
            status: ServerStatus::new(ServerStatusState::InActive),
        })
    }

    pub fn address(&self) -> (String, u16) {
        self.addr.get_tuple()
    }

    pub fn status(&self) -> &ServerStatus {
        &self.status
    }

    pub async fn run(&self) -> IOResult<()> {
        self.status.active();
        let runner = Arc::clone(&self.runner);
        let mut guard = runner.lock().unwrap();

        match (&mut *guard).await {
            Err(e) => {
                self.status.inactive();
                return Err(e);
            }
            _ => (),
        };
        Ok(())
    }

    pub async fn pause(&self) -> Result<(), ServerHandlingError> {
        if self.status.is_idle() {
            return Err(ServerHandlingError::new(
                ServerHandlingErrorKind::ServerAlreadyPause,
                "server already paused",
            ));
        }
        if self.status.is_inactive() {
            return Err(ServerHandlingError::new(
                ServerHandlingErrorKind::ServerStopped,
                "server was stopped",
            ));
        }
        self.status.idle();
        self.handler.pause().await;
        Ok(())
    }
    pub async fn resume(&self) -> Result<(), ServerHandlingError> {
        if self.status.is_active() {
            return Err(ServerHandlingError::new(
                ServerHandlingErrorKind::ServerAlreadyRunning,
                "server already running",
            ));
        }
        if self.status.is_inactive() {
            return Err(ServerHandlingError::new(
                ServerHandlingErrorKind::ServerStopped,
                "server was stopped",
            ));
        }
        self.status.active();
        self.handler.resume().await;
        Ok(())
    }
    pub async fn stop(&self) -> Result<(), ServerHandlingError> {
        if self.status.is_inactive() {
            return Err(ServerHandlingError::new(
                ServerHandlingErrorKind::ServerAlreadyStopped,
                "server already stopped",
            ));
        }
        self.status.inactive();

        self.handler.stop(true).await;
        Ok(())
    }
}

pub struct ServerRunner;

impl ServerRunner {
    pub fn new(addr: ServerAddr) -> IOResult<ActixServer> {
        match HttpServer::new(|| {
            App::new()
            .wrap(Logger::new(
                "[%s] (%r %a) \n  ip: %{r}a\n  time: %Ts,\n  pid: %P,\n  user-agent: %{User-Agent}i,\n  content-type: %{Content-Type}i,\n  size: %bb",
            ))
            .service(web::scope("/api/auth").service(services::auth::login)
        )
            .service(
                web::scope("/api")
                .service(services::traffic::get_traffic)
                .service(services::interface::get_interface)
                .service(services::info::get_info)
                .service(services::config::get_config)
                .service(services::config::edit_config)
                .service(services::daemon::get_daemon_status)
                .service(services::daemon::stop_daemon)
                .service(services::daemon::restart_daemon)
                .wrap(HttpAuthentication::bearer(Auth::validate)),
            ).default_service(route().to(services::not_found::not_found))
        })
        .bind(addr.get_tuple())
            {
                Err(err) => Err(err),
                Ok(server) => {
                    Ok(server.run())
                }
            }
    }
}
#[derive(Clone)]
pub struct ServerAddr {
    ip: String,
    port: u16,
}

impl ServerAddr {
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            ip: ip.to_owned(),
            port,
        }
    }

    pub fn from_config_file() -> Self {
        let configs = app::config::Configs::init().unwrap();
        let (ip, port) = (configs.server().ip(), configs.server().port() as u16);

        Self { ip, port }
    }

    pub fn get_tuple(&self) -> (String, u16) {
        (self.ip.clone(), self.port)
    }
}

#[derive(PartialEq)]
pub enum ServerStatusState {
    Active,
    Idle,
    InActive,
}

impl From<usize> for ServerStatusState {
    fn from(val: usize) -> Self {
        use self::ServerStatusState::*;
        match val {
            0 => Active,
            1 => Idle,
            2 => InActive,
            _ => unreachable!(),
        }
    }
}

impl ToString for ServerStatusState {
    fn to_string(&self) -> String {
        use self::ServerStatusState::*;
        match self {
            Active => "active",
            Idle => "idle",
            InActive => "inactive",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct ServerStatus {
    flag: Arc<AtomicUsize>,
}

impl ServerStatus {
    // use self::ServerStatusState::*;
    pub fn new(state: ServerStatusState) -> Self {
        Self {
            flag: Arc::new(AtomicUsize::new(state as usize)),
        }
    }

    pub fn inactive(&self) {
        self.set_state(ServerStatusState::InActive)
    }
    pub fn idle(&self) {
        self.set_state(ServerStatusState::Idle)
    }
    pub fn active(&self) {
        self.set_state(ServerStatusState::Active)
    }

    #[inline]
    pub fn get_state(&self) -> ServerStatusState {
        self.flag.load(Ordering::SeqCst).into()
    }
    fn set_state(&self, state: ServerStatusState) {
        self.flag.store(state as usize, Ordering::SeqCst)
    }

    pub fn is_active(&self) -> bool {
        self.get_state().eq(&ServerStatusState::Active)
    }
    pub fn is_inactive(&self) -> bool {
        self.get_state().eq(&ServerStatusState::InActive)
    }
    pub fn is_idle(&self) -> bool {
        self.get_state().eq(&ServerStatusState::Idle)
    }
}
