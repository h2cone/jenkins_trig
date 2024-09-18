use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let conf = parse_conf();

    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "{}/view/{}/job/{}/buildWithParameters",
            conf.url, conf.view, conf.job
        ))
        .basic_auth(&conf.user, Some(&conf.token))
        .form(&conf.params)
        .send()
        .await?;

    let loc = resp
        .headers()
        .get("location")
        .expect("No location header found from Jenkins")
        .to_str()?;

    let mut queue_item;
    while {
        queue_item = client
            .get(loc.to_owned() + "/api/json")
            .basic_auth(&conf.user, Some(&conf.token))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        queue_item.get("executable").is_none()
    } {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        println!("Waiting for the job to start...");
    }
    let exec_num = queue_item["executable"]["number"].as_number().unwrap();
    println!("Job started with executable number: {}", exec_num);

    while {
        let job = client
            .get(format!(
                "{}/job/{}/{}/api/json",
                conf.url, conf.job, exec_num
            ))
            .basic_auth(&conf.user, Some(&conf.token))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        match job.get("result") {
            Some(res) => match res.as_str() {
                Some("SUCCESS") => false,
                Some("FAILURE") => panic!("Job failed!"),
                Some("ABORTED") => panic!("Job aborted!"),
                _ => true,
            },
            _ => true,
        }
    } {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        println!("Waiting for the job to finish...");
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Conf {
    #[clap(skip)]
    url: String,
    #[clap(skip)]
    user: String,
    #[clap(skip)]
    token: String,
    #[arg(short, long, default_value = "")]
    view: String,
    #[arg(short, long, default_value = "")]
    job: String,
    #[arg(short, value_parser = parse_key_val::<String, String>, value_delimiter = ';')]
    params: Vec<(String, String)>,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn parse_conf() -> Conf {
    dotenv().ok();
    let mut conf = Conf::parse();

    conf.url = env::var("JENKINS_URL").expect("JENKINS_URL must be set");
    conf.user = env::var("JENKINS_USER").expect("JENKINS_USER must be set");
    conf.token = env::var("JENKINS_TOKEN").expect("JENKINS_TOKEN must be set");

    conf.view.is_empty().then(|| {
        conf.view = env::var("JENKINS_VIEW").expect("JENKINS_VIEW must be set");
    });
    conf.job.is_empty().then(|| {
        conf.job = env::var("JENKINS_JOB").expect("JENKINS_JOB must be set");
    });
    conf
}
