
#[derive(Default)]
pub enum MysqlAuthType {
    Native,
    #[default]
    Sha2,
    ClearPassword
}

#[derive(Default)]
pub enum DB {
    #[default]
    MySQL,
    //placeholders
    PostgreSQL,
    MSSql,

}

pub enum MacroType {
    Create,
    Select,
    Insert
}



