use core_sql::{
    modules::usuarios::insert::insert_user, structs::usuarios::struct_user::CreateUserModel,
};
#[allow(unused_imports)]
mod modules;
mod structs;

#[tokio::main]
async fn main() {
    let user: CreateUserModel = CreateUserModel {
        firstname: "Teste".to_owned(),
        secondname: "Teste".to_owned(),
        email: "teste@test.com".to_owned(),
        password: "teste".to_owned(),
    };
    let _pool = modules::connection::start_connection().await;
    let result = insert_user(&_pool, user).await;
    println!("{:?}", result)
}
