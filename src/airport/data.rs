extern crate reqwest;
extern crate csv;
extern crate time;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use super::{Airport, LatLon, Type, Runway, RunwayIdentifier, Frequencies, Region};

error_chain! {
    errors {
        FieldNotFound(name: String) {
            description("data field not found")
            display("data field {} not found", name)
        }
    }

    foreign_links {
        Reqwest(reqwest::Error);
        Csv(csv::Error);
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
    }
}

const DATA_HOME:        &str = "http://ourairports.com/data";
const AIRPORTS_FILE:    &str = "airports.csv";
const FREQUENCIES_FILE: &str = "airport-frequencies.csv";
const COUNTRIES_FILE:   &str = "countries.csv";
const RUNWAYS_FILE:     &str = "runways.csv";

const DOWNLOAD_DATE_FILE: &str = "month-downloaded";

fn download_data_file(client: &reqwest::Client, name: &str) -> Result<Vec<u8>> {
    println!("downloading {}", name);

    let url = format!("{}/{}", DATA_HOME, name);
    let mut request  = client.get(&url)?.send()?;
    let mut csv_data = Vec::new();

    request.read_to_end(&mut csv_data)?;
    Ok(csv_data)
}

trait HashMapExtras<V> {
    fn get_field(&mut self, name: &str) -> Result<V>;
}

impl<V> HashMapExtras<V> for HashMap<String, V> {
    fn get_field(&mut self, name: &str) -> Result<V> {
        match self.remove(name) {
            Some(v) => Ok(v),
            None    => bail!(ErrorKind::FieldNotFound(name.into())),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Country {
    pub name:   String,
    pub region: Region,
}

pub struct DataFiles {
    data_dir: PathBuf,
}

impl DataFiles {
    pub fn new(data_path: &Path) -> Result<DataFiles> {
        let mut base_dir = env::current_exe()?;
        base_dir.pop();
        base_dir.push(data_path);

        Ok(DataFiles {
            data_dir: base_dir,
        })
    }

    pub fn ensure_updated_data(&self) -> Result<()> {
        let path = self.get_path_in_data(DOWNLOAD_DATE_FILE);

        if path.exists() {
            let mut file  = File::open(path)?;
            let mut month = String::new();

            file.read_to_string(&mut month)?;

            if time::now_utc().tm_mon != month.parse::<i32>()? {
                self.get_new_data()?;
            }
        } else {
            self.get_new_data()?;
        }

        Ok(())
    }

    fn save_data_to_file(&self, name: &str, data: &[u8]) -> Result<()> {
        let path = self.get_path_in_data(name);
        let mut file = File::create(path)?;
        
        file.write_all(data)?;
        Ok(())
    }

    fn download_and_save_data(&self, client: &reqwest::Client, name: &str) -> Result<()> {
        let data = download_data_file(client, name)?;
        self.save_data_to_file(name, &data)
    }

    fn retrieve_runway_data(&self, client: &reqwest::Client) -> Result<()> {
        // The airport runways file contains an extra comma at the end of the headers
        // that breaks CSV parsing, so we need to remove it before saving the file
        let mut runways = download_data_file(&client, RUNWAYS_FILE)?;
        let comma_pos   = runways.windows(2)
                                 .position(|b| b == &[b',', b'\n']);

        match comma_pos {
            Some(comma_pos) => runways[comma_pos] = b' ',
            None => println!("warning: extra comma no longer exists in {} file", RUNWAYS_FILE),
        }

        self.save_data_to_file(RUNWAYS_FILE, &runways)
    }

    pub fn get_new_data(&self) -> Result<()> {
        if !self.data_dir.exists() {
            fs::create_dir(&self.data_dir)?;
        }

        let client = reqwest::Client::new()?;

        self.download_and_save_data(&client, AIRPORTS_FILE)?;
        self.download_and_save_data(&client, FREQUENCIES_FILE)?;
        self.download_and_save_data(&client, COUNTRIES_FILE)?;
        self.retrieve_runway_data(&client)?;

        let mut date_file = File::create(
            self.get_path_in_data(DOWNLOAD_DATE_FILE))?;

        write!(date_file, "{}", time::now_utc().tm_mon)?;

        println!("data download complete");
        Ok(())
    }

    pub fn parse(&self) -> Result<Vec<Airport>> {
        let mut rdr = csv::Reader::from_path(
            self.get_path_in_data(AIRPORTS_FILE))?;

        let mut airports    = Vec::new();
        let mut runways     = self.parse_runways()?;
        let mut frequencies = self.parse_frequencies()?;

        for airport in rdr.deserialize() {
            let mut data: HashMap<String, String> = airport?;
            let icao = data.get_field("ident")?;

            // Get the airport type early so we can skip ones we don't care about (like balloon ports)
            // and save a very small amount of time
            let _type = match Type::parse(&data.get_field("type")?) {
                Some(t) => t,
                None    => continue,
            };

            airports.push(Airport {
                icao: icao.clone(),
                pos: LatLon {
                    lat: data.get_field("latitude_deg")?.parse()?,
                    lon: data.get_field("longitude_deg")?.parse()?,
                },
                _type:       _type,
                runways:     runways.remove(&icao),
                frequencies: frequencies.remove(&icao),
                region: Region {
                    country:   data.get_field("iso_country")?,
                    continent: data.get_field("continent")?,
                },
            });
        }

        Ok(airports)
    }

    fn parse_runways(&self) -> Result<HashMap<String, Vec<Runway>>> {
        let mut rdr = csv::Reader::from_path(
            self.get_path_in_data(RUNWAYS_FILE))?;

        let mut runways = HashMap::new();

        for runway in rdr.deserialize() {
            let mut data: HashMap<String, String> = runway?;

            let icao = data.get_field("airport_ident")?;
            let runway_list = runways.entry(icao).or_insert(Vec::new());

            runway_list.push(Runway {
                ident: RunwayIdentifier {
                    north: data.get_field("le_ident")?,
                    south: data.get_field("he_ident")?,
                },
                width:  data.get_field("width_ft")?.parse().ok(),
                length: data.get_field("length_ft")?.parse().ok(),
                closed: data.get_field("closed")?.parse::<i32>().ok().map(|val| val == 1),
            });
        }

        Ok(runways)
    }

    fn parse_frequencies(&self) -> Result<HashMap<String, Frequencies>> {
        let mut rdr = csv::Reader::from_path(
            self.get_path_in_data(FREQUENCIES_FILE))?;

        let mut frequencies = HashMap::new();
        let mut temp_freqs  = HashMap::new();
        let mut last_icao   = String::new();

        for freq_data in rdr.deserialize() {
            let mut data: HashMap<String, String> = freq_data?;
            let icao = data.get_field("airport_ident")?;

            temp_freqs.insert(
                data.get_field("type")?,
                data.get_field("frequency_mhz")?
            );

            if icao != last_icao {
                let app_dep = temp_freqs.remove("A/D");

                frequencies.insert(last_icao.clone(), Frequencies {
                    ground:    temp_freqs.remove("GND"),
                    tower:     temp_freqs.remove("TWR"),
                    departure: app_dep.clone().or(temp_freqs.remove("DEP")),
                    approach:  app_dep.clone().or(temp_freqs.remove("APP")),
                    atis:      temp_freqs.remove("ATIS"),
                });

                last_icao = icao;
                temp_freqs.clear();
            }
        }

        Ok(frequencies)
    }

    pub fn parse_countries(&self) -> Result<Vec<Country>> {
        let mut rdr = csv::Reader::from_path(
            self.get_path_in_data(COUNTRIES_FILE))?;

        let mut countries = Vec::new();

        for country in rdr.deserialize() {
            let mut data: HashMap<String, String> = country?;

            countries.push(Country {
                name: data.get_field("name")?,
                region: Region {
                    country:   data.get_field("code")?,
                    continent: data.get_field("continent")?,
                },
            });
        }

        Ok(countries)
    }

    fn get_path_in_data(&self, file: &str) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(file);

        path
    }
}