table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        cpf_cnpj -> Varchar,
        password -> Varchar,
        role_id -> Nullable<Int4>,
        active -> Bool,
        verification_code -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
