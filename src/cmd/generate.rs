use std::error::Error;
use std::fs;

use clap::Args;
use colored::Colorize;

use crate::cmd::Execute;

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
    name: hello
    # request url, can use expression, example: http://${var.host}:8080/hello
    url: http://127.0.0.1:8080/hello
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
    #[arg(long, short)]
    pub from: Option<String>,
    #[arg(long, short, default_value = "template.yaml")]
    pub out: Option<String>,
}

impl Execute for GenerateArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self.out {
            None => {}
            Some(out) => {
                return match self.from {
                    Some(_from) => Ok(()),
                    None => {
                        return match fs::write(&out, YURL_TEMPLATE) {
                            Ok(_) => Ok(println!("{}", format!("please view {}", out).green())),
                            Err(e) => Err(Box::new(e)),
                        };
                    }
                };
            }
        }
        Ok(())
    }
}
