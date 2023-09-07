use util::captcha;

mod util;
mod consts;

fn main() {

    match captcha::get_balance(consts::CAPMONSTER_TOKEN) {
        Ok(balance) => println!("{:?}", balance),
        Err(error) => println!("{:?}", error),
    }

}
