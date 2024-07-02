use std::error::Error;
use std::fs;

use clap::Args;
use colored::Colorize;

use crate::cmd::Execute;
use crate::core::request::{ContentType, Method, Request};
use crate::core::Template;
use crate::success;

const YURL_TEMPLATE: &str = r#"# import other yurl yaml file.
# relative or absolute paths can be used
# relative to the current yaml file
# ./ is mandatory
imports:
  - ./var.yaml
# variable, use ${var.name} can obtain.
vars:
  name: tom
  host: 127.0.0.1
# request list
requests:
    # request execution order
  - order: 2
    # request name
    name: example
    # request url, can use expression, example: http://${var.host}:8080/example
    url: http://127.0.0.1:8080/example
    # request method: GET | POST | PUT | DELETE
    method: POST
    # request headers, can use expression.
    headers:
      tenant-id: 10000
      # get variable value expression.
      name: ${var.name}
      # get function value expression, function list can be viewed through [yurl function list].
      uuid: ${fun.uuid}
      # get response value expression, only when the dependent request response_type=JSON.
      # expression describe: ${res.    hello.                             token}
      #                        fixed   dependent request the name         dependent request the response json path
      token: ${res.hello.token}
    # request params, can use expression.
    params:
      name: ${var.name}
      id: ${fun.uuid}
    # request ContentType: URLENCODED | FORM | JSON | FILE
    content_type: JSON
    # response data type: TEXT | JSON | HTML | FILE
    response_type: JSON
"#;

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct GenerateArg {
    #[arg(long, short, default_value = "template.yaml")]
    pub out: String,
    #[arg(long, short)]
    pub url: Option<String>,
    #[arg(short,long, default_value = "full", value_parser = ["get","post","pust","delete","file","full"])]
    pub type_: String,
}

impl Execute for GenerateArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut template: Template = Default::default();
        template.imports.push("./vars.yaml".to_string());
        template.vars.insert("name".to_string(), "tom".to_string());
        match self.type_.as_str() {
            "get" => {
                let mut request: Request = Default::default();
                if let Some(url) = self.url {
                    request.url = url;
                }
                template.requests.push(request);
                let yaml = serde_yaml::to_string(&template)?;
                fs::write(&self.out, yaml)?;
                Ok(success!(format!("please view {}", self.out)))
            }
            "post" => {
                let mut request: Request = Default::default();
                request.method = Method::POST;
                request.content_type = ContentType::JSON;
                if let Some(url) = self.url {
                    request.url = url;
                }
                template.requests.push(request);
                let yaml = serde_yaml::to_string(&template)?;
                fs::write(&self.out, yaml)?;
                Ok(success!(format!("please view {}", self.out)))
            }
            "put" => {
                let mut request: Request = Default::default();
                request.method = Method::PUT;
                request.content_type = ContentType::JSON;
                if let Some(url) = self.url {
                    request.url = url;
                }
                template.requests.push(request);
                let yaml = serde_yaml::to_string(&template)?;
                fs::write(&self.out, yaml)?;
                Ok(success!(format!("please view {}", self.out)))
            }
            "delete" => {
                let mut request: Request = Default::default();
                request.method = Method::DELETE;
                request.content_type = ContentType::JSON;
                if let Some(url) = self.url {
                    request.url = url;
                }
                template.requests.push(request);
                let yaml = serde_yaml::to_string(&template)?;
                fs::write(&self.out, yaml)?;
                Ok(success!(format!("please view {}", self.out)))
            }
            "file" => {
                let mut request: Request = Default::default();
                request.method = Method::POST;
                request.content_type = ContentType::FILE;
                if let Some(url) = self.url {
                    request.url = url;
                }
                template.requests.push(request);
                let yaml = serde_yaml::to_string(&template)?;
                fs::write(&self.out, yaml)?;
                Ok(success!(format!("please view {}", self.out)))
            }
            "full" => match fs::write(&self.out, YURL_TEMPLATE) {
                Ok(_) => Ok(success!(format!("please view {}", self.out))),
                Err(e) => Err(Box::new(e)),
            },
            _ => Ok(()),
        }
    }
}
