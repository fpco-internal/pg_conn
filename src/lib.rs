#[derive(clap::Parser, Clone, Debug)]
#[clap(group = clap::ArgGroup::new("pg_conn_options").conflicts_with("uri").required(false).multiple(true))]
pub struct PgConnOptions {
    #[clap(long, env = "PGHOST", group = "pg_conn_options")]
    pub(crate) host: String,
    #[clap(long, env = "PGPORT", group = "pg_conn_options")]
    pub(crate) port: String,
    #[clap(long, env = "PGDATABASE", group = "pg_conn_options")]
    pub(crate) database: String,
    #[clap(long, env = "PGUSER", group = "pg_conn_options")]
    pub(crate) user: String,
    #[clap(long, env = "PGPASSWORD", group = "pg_conn_options")]
    pub(crate) password: String,
}

#[derive(clap::Parser, Clone, Debug)]
pub struct PgConn {
    #[clap(long)]
    uri: Option<String>,
    #[clap(flatten)]
    options: Option<PgConnOptions>,
}
impl PgConn {
    pub fn get_uri(&self) -> String {
        use percent_encoding::*;
        const CHARS_TO_ESCAPE: &AsciiSet = &CONTROLS
            .add(b'$')
            .add(b'&')
            .add(b'+')
            .add(b',')
            .add(b'/')
            .add(b':')
            .add(b';')
            .add(b'=')
            .add(b'?')
            .add(b'@')
            .add(b' ')
            .add(b'"')
            .add(b'<')
            .add(b'>')
            .add(b'#')
            .add(b'%')
            .add(b'{')
            .add(b'}')
            .add(b'|')
            .add(b'\\')
            .add(b'^')
            .add(b'~')
            .add(b'[')
            .add(b']')
            .add(b'`');
        if let Some(uri) = self.uri.clone() {
            uri
        } else {
            let opt = self.options.clone().unwrap();
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                opt.user,
                utf8_percent_encode(&opt.password, &CHARS_TO_ESCAPE),
                opt.host,
                opt.port,
                opt.database
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn it_works() {
        let cmd = PgConn::command();
        let uri_result =
            cmd.clone()
                .try_get_matches_from(vec!["cmd", "--uri", "postgres://u:p@h:0/d"]);
        eprintln!("{uri_result:?}");
        assert!(uri_result.is_ok());

        let options_result = cmd.clone().try_get_matches_from(vec![
            "cmd",
            "--host",
            "h",
            "--port",
            "0",
            "--database",
            "d",
            "--user",
            "u",
            "--password",
            "p",
        ]);
        eprintln!("{options_result:?}");
        assert!(options_result.is_ok());
    }
    #[test]
    fn it_not_works() {
        let cmd = PgConn::command();
        let missing_result = cmd.clone().try_get_matches_from(vec![
            "cmd",
            "--host",
            "h",
            "--port",
            "0",
            "--user",
            "u",
            "--password",
            "p",
        ]);
        assert!(missing_result.is_err());
        let mixing_result = cmd.clone().try_get_matches_from(vec![
            "cmd",
            "--host",
            "h",
            "--uri",
            "postgres://u:p@h:0/d",
        ]);
        assert!(mixing_result.is_err());
        let both_result = cmd.clone().try_get_matches_from(vec![
            "cmd",
            "--uri",
            "postgres://u:p@h:0/d",
            "--host",
            "h",
            "--port",
            "0",
            "--database",
            "d",
            "--user",
            "u",
            "--password",
            "p",
        ]);
        assert!(both_result.is_err());
    }
}
