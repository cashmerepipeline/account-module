/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-22 07:44
Introduction:
*/

#[derive(Debug)]
enum GroupStatus {
    Active,
    Stop,
}

// #[derive(Debug)]
// pub(crate) struct Group {
//     pub id: String,
//     pub name: String,
//     pub creator_id: String,
//     pub create_timestamp: u64,
//     pub modifier_id: String,
//     pub modify_timestamp: u64,
//     pub owner: String,

//     fields: fields,
// }