use config::Config;
use blob::Blob;

pub struct Server {
    pub config: Config,
    pub blob: Blob,
}
