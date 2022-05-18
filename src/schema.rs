table! {
    auths (id) {
        id -> Uuid,
        user_id -> Uuid,
        payload -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    goals (id) {
        id -> Uuid,
        project_id -> Nullable<Uuid>,
        name -> Varchar,
        description -> Varchar,
        deadline -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        owner_id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        project_id -> Nullable<Uuid>,
        name -> Varchar,
        description -> Varchar,
        done -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        cpf_cnpj -> Varchar,
        password -> Varchar,
        role_id -> Nullable<Int4>,
        active -> Bool,
        verification_payload -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_to_projects (user_id, project_id) {
        user_id -> Uuid,
        project_id -> Uuid,
    }
}

joinable!(auths -> users (user_id));
joinable!(goals -> projects (project_id));
joinable!(tasks -> projects (project_id));
joinable!(users_to_projects -> projects (project_id));
joinable!(users_to_projects -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auths,
    goals,
    projects,
    tasks,
    users,
    users_to_projects,
);
