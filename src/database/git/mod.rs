use crate::config::Config;
use anyhow::Result;
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tempfile::TempDir;

mod parse;
mod theme;
mod update;

pub struct Repo<'repo> {
    repo: Repository,
    tempdir: TempDir,
    fetch_options: FetchOptions<'repo>,
}

impl<'repo> Repo<'repo> {
    pub fn init() -> Result<Self> {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(get_fetch_options());

        let tempdir = TempDir::new()?;

        let repo = builder.clone(&Config::read().git.repository, tempdir.path())?;

        Ok(Self {
            repo,
            tempdir,
            fetch_options: get_fetch_options(),
        })
    }
}

fn get_fetch_options<'repo>() -> FetchOptions<'repo> {
    let mut fetch_options = FetchOptions::new();

    if let Some(proxy_url) = Config::read().git.proxy.as_ref() {
        let mut proxy_option = ProxyOptions::new();
        proxy_option.url(proxy_url);
        fetch_options.proxy_options(proxy_option);
    }

    if let (Some(username), Some(password)) = (
        Config::read().git.user.as_ref(),
        Config::read().git.password.as_ref(),
    ) {
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(username, password));
        fetch_options.remote_callbacks(remote_callbacks);
    }

    fetch_options
}
