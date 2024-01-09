use async_graphql::{EmptySubscription, Schema};
use crate::prelude::*;
use crate::graphql::{mutation::Mutation, query::Query};
use crate::prelude::RaveApiResult;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
