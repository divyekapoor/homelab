fn main() {
    let config_str = include_str!("../data/log4rs.yaml");
    let config = serde_yaml::from_str(config_str).expect(
        "expected config_str to be available");
    log4rs::init_raw_config(config).expect(
        "log4rs initiatlization failed!");

    log::info!("Log4rs config: \n {:}", config_str);
}
