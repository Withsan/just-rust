use anyhow::Result;
use sqlx::{MySql, MySqlPool, QueryBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("init logger error");
    let pool = MySqlPool::connect("mysql://root:telecom@localhost/testdb").await?;
    let transaction = pool.begin().await?;
    let student = Student {
        id: 0,
        name: "ytt".to_string(),
        age: 18,
        class_id: 8,
    };
    let students = (0..10)
        .map(|i| Student {
            id: i,
            name: format!("test_user_{i}"),
            age: (18 + i) as i32,
            class_id: 19,
        })
        .collect::<Vec<_>>();
    insert_bulk(&students, &pool).await?;
    insert(&student, &pool).await?;
    query_all(&pool).await?.iter().for_each(|student| {
        tracing::info!("{student:?}");
    });
    transaction.commit().await?;
    Ok(())
}
async fn insert_bulk(students: &Vec<Student>, pool: &MySqlPool) -> Result<()> {
    let mut query: QueryBuilder<MySql> =
        QueryBuilder::new("insert into students(name,age,class_id)");
    query.push_values(students, |mut builder, student| {
        builder.push_bind(student.name.as_str());
        builder.push_bind(student.age);
        builder.push_bind(student.class_id);
    });
    query.build().execute(pool).await?;
    let student_from_db = query_by_id(90, pool).await?;
    tracing::info!("{student_from_db:?}");
    Ok(())
}
async fn insert(student: &Student, pool: &MySqlPool) -> Result<()> {
    let id = sqlx::query!(
        "insert into students(name,age,class_id) values(?,?,?)",
        student.name,
        student.age,
        student.class_id
    )
    .execute(pool)
    .await?
    .last_insert_id();
    tracing::info!("insert success {id}");
    Ok(())
}
#[derive(Debug)]
struct Student {
    id: i64,
    name: String,
    age: i32,
    class_id: i64,
}
async fn query_all(pool: &MySqlPool) -> Result<Vec<Student>> {
    let students = sqlx::query_as!(Student, "select * from students")
        .fetch_all(pool)
        .await?;
    Ok(students)
}
async fn query_by_id(id: i32, pool: &MySqlPool) -> Result<Student> {
    let student = sqlx::query_as!(Student, "select * from students where id = ?", id)
        .fetch_one(pool)
        .await?;
    Ok(student)
}
