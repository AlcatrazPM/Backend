pub enum UserId {
    ObjectId(bson::oid::ObjectId),
    Email(String),
}