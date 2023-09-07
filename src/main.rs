use util::sms::{Country, Service};

mod util;
mod consts;

fn main() {

    match util::sms::request_phone_number(consts::SMS_MAN_TOKEN, Country::Afghanistan, Service::Discord) {
        Ok(phone_number) => println!("{:?}", phone_number),
        Err(error) => println!("{:?}", error),
    }

}
