use iron::prelude::*;
use router::Router;
use hyper::status::StatusCode;
use postgres::types::ToSql;
use std::str::FromStr;
use std::i32;

use super::request_body;
use ::api::authorization::Authorizer;
use ::proto::response::*;
use ::proto::error::*;
use ::proto::schema::*;
use ::db::schema::*;
use ::db::*;

pub fn get_rulesets(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    let rulesets = conn.query(&RuleSet::select_builder()
                   .filter("ManagerPersonID = $1")
                   .build(),
               &[&user.id])
        .unwrap()
        .into_iter()
        .map(RuleSet::from)
        .collect::<Vec<RuleSet>>();

    Ok(rulesets.as_response())
}

pub fn get_ruleset(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    if !user.roles.Manager {
        return Err(NotAuthorizedError::from_str("Only manager can access rulesets").into());
    }

    let ruleset_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No RuleSet ID found in request");

    let ruleset = conn.query(&RuleSet::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&ruleset_id])
        .unwrap()
        .into_iter()
        .last()
        .map(RuleSet::from);

    match ruleset {
        Some(ruleset) => Ok(ruleset.as_response()),
        None => Err(NotFoundError::from_str(format!("RuleSet {} not found", ruleset_id)).into()),
    }
}

pub fn put_ruleset(req: &mut Request) -> IronResult<Response> {
    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    if !user.roles.Manager {
        return Err(NotAuthorizedError::from_str("Only manager can create rulesets").into());
    }

    let new_ruleset: NewRuleSet = request_body(req)?;
    let ruleset = RuleSet {
        ID: 0,
        ManagerPersonID: Some(user.id),
        Name: new_ruleset.Name,
        Body: new_ruleset.Body,
        IsDefault: false
    };

    conn.execute(&RuleSet::insert_query(), &ruleset.insert_args())
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}

pub fn update_ruleset(req: &mut Request) -> IronResult<Response> {
    let ruleset_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No RuleSet ID found in request");

    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    if !user.roles.Manager {
        return Err(NotAuthorizedError::from_str("Only manager can create rulesets").into());
    }

    let update_ruleset: UpdateRuleSet = request_body(req)?;

    let mut update = RuleSet::update_builder().filter(format!("ID = {}", ruleset_id));
    let mut values: Vec<&ToSql> = Vec::new();

    let old_ruleset = conn.query(&RuleSet::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&ruleset_id])
        .unwrap()
        .into_iter()
        .map(RuleSet::from)
        .last()
        .ok_or(box NotFoundError::from_str("No such ruleset") as Box<ApiError>)?;

    if let Some(manager_id) = old_ruleset.ManagerPersonID {
        if manager_id != user.id {
            return Err(NotAuthorizedError::from_str("Only manager that created ruleset can edit \
                                                     it")
                .into());
        }
    } else {
        update = update.set("ManagerPersonID");
        values.push(&user.id);
    }

    if let Some(name) = update_ruleset.Name.as_ref() {
        update = update.set("Name");
        values.push(name);
    }

    if let Some(body) = update_ruleset.Body.as_ref() {
        update = update.set("Body");
        values.push(body);
    }

    // Early exit if we got empty json
    if values.is_empty() {
        return Ok(Response::with(StatusCode::Ok));
    }

    conn.execute(&update.build(), &values)
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}

pub fn delete_ruleset(req: &mut Request) -> IronResult<Response> {
    let ruleset_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("cnt")
        .map(|s| i32::from_str(s).unwrap())
        .expect("No RuleSet ID found in request");

    let conn = get_db_connection();
    let user = Authorizer::authorize_request(&conn, req)?;

    let old_ruleset = conn.query(&RuleSet::select_builder()
                   .filter("ID = $1")
                   .build(),
               &[&ruleset_id])
        .unwrap()
        .into_iter()
        .map(RuleSet::from)
        .last()
        .ok_or(box NotFoundError::from_str("No such ruleset") as Box<ApiError>)?;

    if old_ruleset.ManagerPersonID.map_or(true, |manager_id| manager_id == user.id) {
        return Err(NotAuthorizedError::from_str("Only creator of ruleset can delete it").into());
    }

    conn.execute(&RuleSet::delete_builder()
                     .filter("ID = $1")
                     .build(),
                 &[&ruleset_id])
        .unwrap();

    Ok(Response::with(StatusCode::Ok))
}
