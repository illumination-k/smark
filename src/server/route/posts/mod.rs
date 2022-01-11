pub mod search;

use actix_web::{get, web, HttpRequest, HttpResponse};
use itertools::Itertools;
use serde::Deserialize;
use std::ops::Deref;
use tantivy::{
    query::{AllQuery, BooleanQuery, Occur, Query, TermQuery},
    schema::IndexRecordOption,
    Index, Term,
};

use crate::text_engine::{
    query::get_all,
    schema::{FieldGetter, PostField},
};

#[derive(Debug, Deserialize)]
pub struct GetPostsQueryParams {
    lang: Option<String>,
    category: Option<String>,
    tag: Option<String>,
}

#[get("/posts")]
async fn get_posts(index: web::Data<Index>, req: HttpRequest) -> HttpResponse {
    let index = index.into_inner();
    let schema = index.schema();
    let fb = FieldGetter::new(schema);
    let params = web::Query::<GetPostsQueryParams>::from_query(req.query_string()).unwrap();

    let mut queries = vec![];
    if let Some(lang) = params.lang.to_owned() {
        let lang_field = fb.get_field(PostField::Lang);
        let term = Term::from_field_text(lang_field, &lang);
        let query: Box<dyn Query> = Box::new(TermQuery::new(term, IndexRecordOption::Basic));
        queries.push((Occur::Must, query));
    }

    if let Some(category) = params.category.to_owned() {
        let category_field = fb.get_field(PostField::Category);
        let term = Term::from_field_text(category_field, &category);
        let query: Box<dyn Query> = Box::new(TermQuery::new(term, IndexRecordOption::Basic));

        queries.push((Occur::Must, query));
    }

    if let Some(tag) = params.tag.to_owned() {
        let tag_field = fb.get_field(PostField::Tags);
        let term = Term::from_field_text(tag_field, &tag);
        let query: Box<dyn Query> = Box::new(TermQuery::new(term, IndexRecordOption::Basic));

        queries.push((Occur::Must, query));
    }

    let docs = if queries.is_empty() {
        let q: Box<dyn Query> = Box::new(AllQuery {});
        get_all(&q, index.deref())
    } else {
        let q: Box<dyn Query> = Box::new(BooleanQuery::new(queries));
        get_all(&q, index.deref())
    }
    .unwrap()
    .iter()
    .map(|doc| index.schema().to_named_doc(doc))
    .collect_vec();

    HttpResponse::Ok().json(docs)
}