use reqwest::Error as ReqwestError;
use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// Service enum
///
/// Details: https://sms-man.com/api
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum Service {
    Google = 122,
    Facebook = 124,
    Twitter = 125,
    Whatsapp = 6,
    Discord = 162,
}

/// Country enum
///
/// Details: https://sms-man.com/api
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum Country {
    RussianFederation = 1,
    Kazakhstan = 2,
    China = 3,
    Ukraine = 4,
    USA = 5,
    Malaysia = 6,
    Indonesia = 7,
    Philippines = 8,
    Myanmar = 9,
    Vietnam = 10,
    Romania = 11,
    Poland = 12,
    Canada = 13,
    India = 14,
    Zambia = 15,
    Pakistan = 16,
    Bangladesh = 17,
    Mexico = 18,
    Cambodia = 19,
    Nicaragua = 20,
    Kenya = 96,
    Kyrgyzstan = 97,
    Israel = 98,
    HongKong = 99,
    UnitedKingdom = 100,
    Madagascar = 101,
    Congo = 102,
    Nigeria = 103,
    Macao = 104,
    Egypt = 105,
    Ireland = 106,
    Laos = 107,
    Haiti = 108,
    CoteDIvoire = 109,
    Gambia = 110,
    Serbia = 111,
    Yemen = 112,
    SouthAfrica = 113,
    Colombia = 114,
    Estonia = 115,
    Azerbaijan = 116,
    Morocco = 117,
    Ghana = 118,
    Argentina = 119,
    Uzbekistan = 120,
    Cameroon = 121,
    Chad = 122,
    Germany = 123,
    Lithuania = 124,
    Croatia = 125,
    Sweden = 126,
    Iraq = 127,
    Netherlands = 128,
    Latvia = 129,
    Austria = 130,
    Belarus = 131,
    Thailand = 132,
    SaudiArabia = 133,
    Taiwan = 134,
    Spain = 135,
    Iran = 136,
    Algeria = 137,
    Slovenia = 138,
    Senegal = 139,
    Turkey = 140,
    Czechia = 141,
    SriLanka = 142,
    Peru = 143,
    NewZealand = 144,
    Guinea = 145,
    Mali = 146,
    Venezuela = 147,
    Ethiopia = 148,
    Mongolia = 149,
    Brazil = 150,
    Afghanistan = 151,
    Uganda = 152,
    Angola = 153,
    Cyprus = 154,
    France = 155,
    PapuaNewGuinea = 156,
    Mozambique = 157,
    Nepal = 158,
    Belgium = 159,
    Bulgaria = 160,
    Hungary = 161,
    Moldova = 162,
    Italy = 163,
    Paraguay = 164,
    Honduras = 165,
    Tunisia = 166,
    Somalia = 167,
    TimorLeste = 168,
    Bolivia = 169,
    CostaRica = 170,
    Guatemala = 171,
    UnitedArabEmirates = 172,
    Zimbabwe = 173,
    PuertoRico = 174,
    Sudan = 175,
    Togo = 176,
    DRCongo = 177,
    Albania = 178,
    AmericanSamoa = 179,
    AntiguaAndBarbuda = 182,
    Armenia = 183,
    Australia = 185,
    Bahamas = 186,
    Bahrain = 187,
    Barbados = 188,
    Belize = 189,
    Benin = 190,
    Bhutan = 192,
    BosniaAndHerzegovina = 193,
    Botswana = 194,
    BurkinaFaso = 196,
    Burundi = 197,
    CaboVerde = 198,
    CentralAfricanRepublic = 200,
    Chile = 201,
    Comoros = 203,
    Cuba = 205,
    Denmark = 206,
    DominicanRepublic = 209,
    Ecuador = 210,
    ElSalvador = 211,
    EquatorialGuinea = 212,
    FaroeIslands = 214,
    Finland = 216,
    FrenchGuiana = 217,
    Gabon = 219,
    Georgia = 220,
    Greece = 222,
    Grenada = 224,
    Guadeloupe = 225,
    GuineaBissau = 227,
    Guyana = 228,
    Iceland = 229,
    Jamaica = 230,
    Jordan = 232,
    Kuwait = 234,
    Lebanon = 235,
    Lesotho = 236,
    Liberia = 237,
    Luxembourg = 239,
    Malawi = 240,
    Maldives = 241,
    Martinique = 244,
    Mauritania = 245,
    Mauritius = 246,
    Namibia = 251,
    NewCaledonia = 254,
    Niger = 255,
    Norway = 259,
    Oman = 260,
    Portugal = 263,
    Qatar = 264,
    Rwanda = 265,
    Singapore = 270,
    Slovakia = 271,
    SolomonIslands = 272,
    Suriname = 274,
    Switzerland = 276,
    Tajikistan = 277,
    TrinidadAndTobago = 280,
    Turkmenistan = 281,
    TurksAndCaicosIslands = 282,
    Uruguay = 284,
    KoreaRepublicOf = 296,
    Libya = 297,
    NorthMacedonia = 298,
    SaintKittsAndNevis = 305,
    SaintLucia = 306,
    SaintVincentAndTheGrenadines = 309,
    Syria = 312,
    Tanzania = 313,
    Swaziland = 315,
    Panama = 316,
    SierraLeone = 317,
    Reunion = 319,
    RepublicOfSouthSudan = 320,
}

/// ApiError struct
///
/// Description: Provides handling for generic API errors
/// As noted on documentation, example being:
///
/// # Example
/// {"success":false,"error_code":"wrong_token","error_msg": {"token": "Wrong token!"}}
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub error_code: String,
    pub error_msg: String,
}

/// SmsError enum
///
/// Description: Cuter way to handle errors from both reqwest and the API
///
#[derive(Debug)]
pub enum SmsError {
    Api(ApiError),
    Reqwest(ReqwestError), // Change the variant to hold reqwest::Error
}

/// Implement Display for SmsError
///
/// Description: Sexy logging for SmsError :3
/// yeah that's about it
///
impl fmt::Display for SmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmsError::Api(api_error) => write!(
                f,
                "ApiError: error_code={}, error_msg={}",
                api_error.error_code, api_error.error_msg
            ),
            SmsError::Reqwest(reqwest_error) => write!(f, "ReqwestError: {}", reqwest_error),
        }
    }
}

/// Implement the Error trait for the custom SmsError type, which allows it to be used as an error type.
impl Error for SmsError {}

/// Implement the conversion from ReqwestError to SmsError.
impl From<ReqwestError> for SmsError {
    fn from(error: ReqwestError) -> Self {
        SmsError::Reqwest(error)
    }
}

/// Represents information about the user's balance from the API.
#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    pub balance: String, // The user's balance as a String.
    pub channels: f64,   // The number of channels as a floating-point number.
    pub hold: f64,       // The hold amount as a floating-point number.
}

/// Represent information about a phone number object from the API.
#[derive(Debug, Deserialize, Serialize)]
pub struct PhoneNumber {
    pub request_id: i32,     // An identifier for the request.
    pub country_id: i32,     // The country identifier as an integer. -> Country
    pub application_id: i32, // The application identifier as an integer. -> Service
    pub number: String,      // The phone number as a String.
}

/// Returns a Balance struct containing the user's balance information.
///
/// # Arguments
///
/// * `token` - A string slice that holds the user's API token.
///
/// # Example
///
/// ```
/// use util::sms::get_balance;
/// let balance = get_balance("token");
///
/// match balance {
///    Ok(balance) => println!("{:?}", balance),
///   Err(error) => println!("{:?}", error),
/// }
/// ```
pub fn get_balance(token: &str) -> Result<Balance, SmsError> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://api.sms-man.com/control/get-balance?token={}", token);
    let response = client.get(&url).send()?;

    drop(client);

    if response.status().is_success() {
        let balance: Balance = response.json()?;
        Ok(balance)
    } else {
        let error: ApiError = response.json()?;
        Err(SmsError::Api(error))
    }
}

/// Returns a PhoneNumber struct containing the generated phone number information.
///
/// # Arguments
///
/// * `token` - A string slice that holds the user's API token.
/// * `country_id` - A Country enum that holds the country identifier.
/// * `service` - A Service enum that holds the application identifier.
///
/// # Example
///
/// ```
/// use util::sms::{Country, Service};
/// use util::sms::request_phone_number;
///
/// let phone_number = request_phone_number("token", Country::Afghanistan, Service::Discord);
///
/// match phone_number {
///   Ok(phone_number) => println!("{:?}", phone_number),
///  Err(error) => println!("{:?}", error),
/// }
/// ```
pub fn request_phone_number(
    token: &str,
    country_id: Country,
    service: Service,
) -> Result<PhoneNumber, SmsError> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "http://api.sms-man.com/control/get-number?token={}&country_id={}&application_id={}",
        token, country_id as i32, service as i32
    );
    let response = client.get(&url).send()?;

    drop(client);

    if response.status().is_success() {
        let phone_number: PhoneNumber = response.json()?;
        Ok(phone_number)
    } else {
        let error: ApiError = response.json()?;
        Err(SmsError::Api(error))
    }
}
