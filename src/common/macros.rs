#[macro_export]
macro_rules! query_primary_key {
    () => {
        "int not null PRIMARY KEY AUTO_INCREMENT"
    };
}