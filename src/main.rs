use clap::{Arg, App};
use reqwest;
use std::{ env, fmt, fs };
use std::io::{self, BufRead};

// CONFIG
fn set_env(file: &str) {
  // set env values from file at path provided
  // this is not super sophisticated and probably is missing some error checking
  let env_file = fs::read_to_string(file).expect("Cannot read env file at path provided");
    let lines = env_file.lines();
    for line in lines {
      if !line.starts_with("#") && line.contains("=") {
        let v: Vec<&str> = line.split("=").collect();
        env::set_var(v[0], v[1])
      }
    }
}

// ERROR HANDLING

#[derive(Debug)]
pub enum APIError {
  BadRequest(String),
  Unauthorized(String),
  Forbidden(String),
  TooManyRequests(String),
  ServerError(String),
  Unknown
}

impl fmt::Display for APIError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        APIError::BadRequest(v) => v.fmt(f),
        APIError::Unauthorized(v) => v.fmt(f),
        APIError::Forbidden(v) => v.fmt(f),
        APIError::TooManyRequests(v) => v.fmt(f),
        APIError::ServerError(v) => v.fmt(f),
        APIError::Unknown => "Unknown error".fmt(f)
      }
  }
}

// Try to provide helpful error messages when something goes wrong with the API calls
// In theory the error codes should be standard so we can use this for everything
fn check_status(code: reqwest::StatusCode) -> Result<bool,APIError> {

  match code.as_str() {
    "400" => Err(APIError::BadRequest(String::from("Bad request: did you provide API credentials as env values?"))),
    "401" => Err(APIError::Unauthorized(String::from("Unauthorized: check your credentials are correct"))),
    "403" => Err(APIError::Forbidden(String::from("Access forbidden: have you been banned?"))),
    "423" => Err(APIError::TooManyRequests(String::from("Too many requests error"))),
    _ if code.as_str().starts_with("5") => Err(APIError::ServerError(String::from("Server error: it's not you, it's them"))),
    _ => Err(APIError::Unknown)
  }
}

// TWITTER
fn tweet(msg: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {

  // We need a custom struct for oauth
  #[derive(oauth::Request)]
  struct Tweet<'a> {
      status: &'a str,
  }
  // our actual message
  let post = Tweet {
    status: &msg
  };
  
  // Twitter status posting endpoint
  let endpoint = "https://api.twitter.com/1.1/statuses/update.json";

  // We have to send the status as a URL encoded param instead of form data
  // because there is a discrepency between Twitter and the oauth standard
  // when asterisks are encoded (or not encoded!)
  let encoded_url = oauth::to_uri_query(String::from(endpoint), &post);
  let client = reqwest::blocking::Client::new();

  // Create Twitter token
  let consumer_key = env::var("TWITTER_CONSUMER_KEY").unwrap_or(String::from(""));
  let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").unwrap_or(String::from(""));
  let access_token = env::var("TWITTER_ACCESS_TOKEN").unwrap_or(String::from(""));
  let token_secret = env::var("TWITTER_ACCESS_SECRET").unwrap_or(String::from(""));
  // this makes a hash of the four secrets
  let hash = oauth::Token::from_parts(consumer_key, consumer_secret, access_token, token_secret);

  // Create the `Authorization` header.
  let authorization_header = oauth::post(endpoint, &post, &hash, oauth::HmacSha1);

  // Let's tweet!
  client.post(encoded_url)
  .header(reqwest::header::AUTHORIZATION, authorization_header)
  .send()
}

// MASTODON
fn toot(msg: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {

  // mastodon API access simply requires a token
  let access_token = env::var("MASTODON_ACCESS_TOKEN").unwrap_or(String::from(""));
  let mut token = String::from("Bearer ");
  token.push_str(&access_token);
  // because it's federated we need to build the url to send the API call to
  let mastodon_base_url = env::var("MASTODON_BASE_URL").unwrap_or(String::from(""));
  let mut endpoint = String::from(&mastodon_base_url);
  endpoint.push_str("/api/v1/statuses");

  // Let's toot!
  let params = [("status", msg)];
  let client = reqwest::blocking::Client::new();
  client.post(&endpoint)
  .form(&params)
  .header(reqwest::header::AUTHORIZATION, token)
  .send()
}

// YAWPING
fn process_yawp(yawp: &String, arguments: clap::ArgMatches) {
  
  // if there are errors we want to 
  // print to sterr and not to stdout
  let mut has_error = false;

  // mastodon
  if arguments.is_present("mastodon") {
    let m_res = toot(&yawp);
    match m_res {
      Ok(resp) => if resp.status().as_str() != "200" { 
        has_error = true;
        let r = check_status(resp.status());
        match r {
          Ok(val) => eprintln!("{}", val),
          Err(err) => eprintln!("Mastodon Error: {}", err)
        }
      },
      Err(err) => { has_error = true; eprintln!("Request Error: {}", err) }
    }
  }

  // twitter
  if arguments.is_present("twitter") {
    let m_res = tweet(&yawp);
    match m_res {
      Ok(resp) => if resp.status().as_str() != "200" { 
        has_error = true;
        let r = check_status(resp.status());
        match r {
          Ok(val) => eprintln!("{}", val),
          Err(err) => eprintln!("Twitter Error: {}", err)
        }
      },
      Err(err) => { has_error = true; eprintln!("Request Error: {}", err) }
    }
  }

  // output
  if !has_error && !arguments.is_present("quiet") {
    println!("{}", &yawp)
  } 

}

 // main
fn main() {

  // get arguments using clap
  let arguments = App::new("lette.rs")
  .version("0.1.1")
  .author("Hugh Rundle")
  .about("Send social media messages from the command line")
  .arg(Arg::with_name("YAWP")
  .help("Message (post) to send. If stdin has been redirected (e.g. via a pipe) YAWP must be provided as '-'. If you are not redirecting stdin, providing a single dash (-) as the YAWP value will cause yawp to hang.")
  .required(true)
  .takes_value(true)
  )
  .arg(Arg::with_name("env")
      .help("path to env file")
      .long("env")
      .short("e")
      .required(false)
      .takes_value(true)
      )
  .arg(Arg::with_name("mastodon")
      .help("Send toot")
      .long("mastodon")
      .short("m")
      .required(false)
      .takes_value(false)
      )
  .arg(Arg::with_name("twitter")
      .help("Send tweet")
      .long("twitter")
      .short("t")
      .required(false)
      .takes_value(false)
      )
  .arg(Arg::with_name("quiet")
      .help("Suppress output (error messages will still be sent to stderr)")
      .long("quiet")
      .short("q")
      .required(false)
      .takes_value(false)
      )
  .get_matches();

    let mut yawp = String::new();

    // if yawp text was provided with the call, just use that
    if arguments.value_of("YAWP").unwrap() != "-" {
      yawp += arguments.value_of("YAWP").unwrap()
    } else {

      // get message value from stdin
      // NOTE: if user provides '-' as arg for YAWP without redirecting stdin
      // from elsewhere this will hang because it never finishes reading potential new lines

      let stdin = io::stdin();
      yawp += &stdin.lock().lines().next().unwrap().unwrap();
      let input = stdin.lock().lines();
      for line in input {
        let l = line.unwrap();
        yawp += "\n";
        yawp += &l;
      }
    }

    // if env value provided, read env values from that file
    if arguments.is_present("env") {
      set_env(arguments.value_of("env").unwrap())
    }
    process_yawp(&yawp, arguments)
}